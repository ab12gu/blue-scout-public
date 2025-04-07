extern crate proc_macro;

use proc_macro2::Span;
use quote::quote_spanned;
use syn::parse::{Parse, ParseStream, Result};
use syn::spanned::Spanned;
use syn::{
    Expr, ExprLit, ExprUnary, Ident, Lit, LitBool, LitFloat, LitInt, LitStr, Token, UnOp, braced,
    bracketed, token,
};

#[allow(clippy::large_enum_variant)]
pub enum JsonValueInput {
    Null(Span),
    Bool(LitBool),
    Number(Lit),
    String(LitStr),
    Array(Vec<JsonValueInput>, Span),
    Object(Vec<(LitStr, JsonValueInput)>, Span),
    Expr(Expr),
    ExtendObject {
        target: Expr,
        properties: Vec<(LitStr, JsonValueInput)>,
        span: Span,
    },
}

struct JsonObjectEntry {
    key: LitStr,
    value: JsonValueInput,
}

impl Parse for JsonObjectEntry {
    fn parse(input: ParseStream) -> Result<Self> {
        let key: LitStr = input.parse()?;
        input.parse::<Token![:]>()?;
        let value: JsonValueInput = input.parse()?;
        Ok(JsonObjectEntry { key, value })
    }
}

fn get_numeric_literal(expr: &Expr) -> Option<(Lit, bool)> {
    match expr {
        Expr::Lit(ExprLit { lit, .. }) => match lit {
            Lit::Int(_) | Lit::Float(_) => Some((lit.clone(), false)),
            _ => None,
        },
        Expr::Unary(ExprUnary {
            op: UnOp::Neg(_),
            expr: inner_expr,
            ..
        }) => {
            if let Expr::Lit(ExprLit { lit, .. }) = &**inner_expr {
                match lit {
                    Lit::Int(_) | Lit::Float(_) => Some((lit.clone(), true)),
                    _ => None,
                }
            } else {
                None
            }
        }
        _ => None,
    }
}

impl Parse for JsonValueInput {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Token![-]) && (input.peek2(LitInt) || input.peek2(LitFloat)) {
            let expr: Expr = input.parse()?;
            if let Some((_, _negated)) = get_numeric_literal(&expr) {
                return Ok(JsonValueInput::Expr(expr));
            } else {
                return Err(syn::Error::new(
                    expr.span(),
                    "Expected negative number literal",
                ));
            }
        }

        let lookahead = input.lookahead1();

        if lookahead.peek(token::Brace) {
            let content;
            let brace_token = braced!(content in input);
            let entries: syn::punctuated::Punctuated<JsonObjectEntry, Token![,]> =
                content.parse_terminated(JsonObjectEntry::parse, Token![,])?;
            Ok(JsonValueInput::Object(
                entries.into_iter().map(|e| (e.key, e.value)).collect(),
                brace_token.span.join(),
            ))
        } else if lookahead.peek(token::Bracket) {
            let content;
            let bracket_token = bracketed!(content in input);
            let elements: syn::punctuated::Punctuated<JsonValueInput, Token![,]> =
                content.parse_terminated(JsonValueInput::parse, Token![,])?;
            Ok(JsonValueInput::Array(
                elements.into_iter().collect(),
                bracket_token.span.join(),
            ))
        } else if lookahead.peek(LitStr) {
            Ok(JsonValueInput::String(input.parse()?))
        } else if lookahead.peek(LitInt) || lookahead.peek(LitFloat) {
            Ok(JsonValueInput::Number(input.parse()?))
        } else if lookahead.peek(Ident) {
            let ident: Ident = input.parse()?;
            let ident_str = ident.to_string();

            if ident_str == "extend" {
                // Parse the "extend: my_object, { ... }" syntax
                input.parse::<Token![:]>()?;
                let target: Expr = input.parse()?;
                input.parse::<Token![,]>()?;

                let content;
                let brace_token = braced!(content in input);
                let entries: syn::punctuated::Punctuated<JsonObjectEntry, Token![,]> =
                    content.parse_terminated(JsonObjectEntry::parse, Token![,])?;

                return Ok(JsonValueInput::ExtendObject {
                    target,
                    properties: entries.into_iter().map(|e| (e.key, e.value)).collect(),
                    span: brace_token.span.join(),
                });
            } else if ident_str == "true" {
                Ok(JsonValueInput::Bool(LitBool::new(true, ident.span())))
            } else if ident_str == "false" {
                Ok(JsonValueInput::Bool(LitBool::new(false, ident.span())))
            } else if ident_str == "null" {
                Ok(JsonValueInput::Null(ident.span()))
            } else {
                // Assume it's the start of a more complex expression
                let mut expr: Expr = Expr::Path(syn::ExprPath {
                    attrs: Vec::new(),
                    qself: None,
                    path: ident.into(),
                });

                // Check for potential continuations like field access or function calls
                while input.peek(Token![.])
                    || input.peek(token::Paren)
                    || input.peek(token::Bracket)
                {
                    if input.peek(Token![.]) && !input.peek(Token![..]) {
                        // Field access
                        let dot_token = input.parse::<Token![.]>()?;
                        let member = input.parse::<syn::Member>()?;
                        expr = Expr::Field(syn::ExprField {
                            attrs: Vec::new(),
                            base: Box::new(expr),
                            dot_token,
                            member,
                        });
                    } else if input.peek(token::Paren) {
                        // Function call
                        let content;
                        let paren_token = syn::parenthesized!(content in input);
                        let args = content.parse_terminated(Expr::parse, Token![,])?;
                        expr = Expr::Call(syn::ExprCall {
                            attrs: Vec::new(),
                            func: Box::new(expr),
                            paren_token,
                            args,
                        });
                    } else if input.peek(token::Bracket) {
                        // Index
                        let content;
                        let bracket_token = bracketed!(content in input);
                        let index = content.parse::<Expr>()?;
                        expr = Expr::Index(syn::ExprIndex {
                            attrs: Vec::new(),
                            expr: Box::new(expr),
                            bracket_token,
                            index: Box::new(index),
                        });
                    } else {
                        break; // Should not happen based on peek checks
                    }
                }
                Ok(JsonValueInput::Expr(expr))
            }
        } else {
            match input.parse::<Expr>() {
                Ok(expr) => Ok(JsonValueInput::Expr(expr)),
                Err(_) => Err(lookahead.error()),
            }
        }
    }
}

pub fn generate_js_value_code(value: JsonValueInput) -> proc_macro2::TokenStream {
    match value {
        JsonValueInput::Null(span) => {
            quote_spanned! {span=> wasm_bindgen::JsValue::NULL }
        }
        JsonValueInput::Bool(b) => {
            let span = b.span();
            quote_spanned! {span=> wasm_bindgen::JsValue::from(#b) }
        }
        JsonValueInput::Number(lit) => {
            let span = lit.span();
            quote_spanned! {span=> wasm_bindgen::JsValue::from(#lit) }
        }
        JsonValueInput::String(s) => {
            let span = s.span();
            quote_spanned! {span=> wasm_bindgen::JsValue::from(#s) }
        }
        JsonValueInput::Array(elements, span) => {
            let element_code = elements.into_iter().map(generate_js_value_code);
            quote_spanned! {span=>
                {
                    let array = js_sys::Array::new();
                    #(
                        array.push(&#element_code);
                    )*
                    wasm_bindgen::JsValue::from(array)
                }
            }
        }
        JsonValueInput::Object(entries, span) => {
            let entry_code = entries.into_iter().map(|(key, value)| {
                let key_span = key.span();
                let value_code = generate_js_value_code(value);
                quote_spanned! {key_span=>
                    js_sys::Reflect::set(
                        &object,
                        &wasm_bindgen::JsValue::from(#key),
                        &#value_code,
                    ).expect("Object should be an object"); // Consider error handling
                }
            });
            quote_spanned! {span=>
                {
                    let object = js_sys::Object::new();
                    #(
                        #entry_code
                    )*
                    wasm_bindgen::JsValue::from(object)
                }
            }
        }
        JsonValueInput::Expr(expr) => {
            let span = expr.span();
            quote_spanned! {span=> wasm_bindgen::JsValue::from(#expr) }
        }
        JsonValueInput::ExtendObject {
            target,
            properties,
            span,
        } => {
            let entry_code = properties.into_iter().map(|(key, value)| {
                let key_span = key.span();
                let value_code = generate_js_value_code(value);
                quote_spanned! {key_span=>
                    js_sys::Reflect::set(
                        &#target,
                        &wasm_bindgen::JsValue::from(#key),
                        &#value_code,
                    ).unwrap(); // Consider error handling
                }
            });

            quote_spanned! {span=>
                {
                    #(
                        #entry_code
                    )*
                    wasm_bindgen::JsValue::from(#target)
                }
            }
        }
    }
}
