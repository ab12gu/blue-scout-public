#![feature(let_chains)]
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use std::fmt::Write as _;
use syn::{
    Expr, Ident, LitStr, Result, Token, Type, TypePath,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token::Comma,
};

// Define a struct for a field declaration with optional pretty name
struct FieldDecl {
    name: Ident,
    ty: Type,
    pretty_name: Option<LitStr>,
}

// Define the overall input structure for the struct definition
struct StructInput {
    name: Ident,
    fields: Punctuated<FieldDecl, Comma>,
}

// Implement parsing for field declaration with pretty name support
impl Parse for FieldDecl {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse()?;
        input.parse::<Token![:]>()?;
        let ty = input.parse()?;

        // Check if there's a pretty name (=>)
        let pretty_name = if input.peek(Token![=>]) {
            input.parse::<Token![=>]>()?;
            Some(input.parse()?)
        } else {
            None
        };

        Ok(FieldDecl {
            name,
            ty,
            pretty_name,
        })
    }
}

// Implement parsing for the struct input
impl Parse for StructInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse()?;
        input.parse::<Token![,]>()?;

        let fields = Punctuated::parse_terminated(input)?;

        Ok(StructInput { name, fields })
    }
}

// Helper function to determine the DataType variant for a field type
fn get_data_type_variant(ty: &Type) -> Option<Ident> {
    if let Type::Path(TypePath { path, .. }) = ty {
        if let Some(segment) = path.segments.last() {
            let type_name = segment.ident.to_string();
            let variant = match type_name.as_str() {
                "u16" => "U16",
                "u32" => "U32",
                "u64" => "U64",
                "i16" => "I16",
                "i32" => "I32",
                "i64" => "I64",
                "String" => "String",
                "bool" => "Bool",
                "f32" => "Float",
                _ => "Unknown",
            };

            return Some(Ident::new(variant, segment.ident.span()));
        }
    }

    None
}

// Helper function to map Rust type to SQL type
fn map_to_sql_type(ty: &Type) -> &'static str {
    if let Type::Path(TypePath { path, .. }) = ty {
        if let Some(segment) = path.segments.last() {
            let type_name = segment.ident.to_string();
            match type_name.as_str() {
                "u16" => "USMALLINT",
                "u32" => "UINTEGER",
                "u64" => "UBIGINT",
                "i16" => "SMALLINT",
                "i32" => "INTEGER",
                "i64" => "BIGINT",
                "String" => "TEXT",
                "bool" => "BOOL",
                "f32" => "FLOAT",
                "f64" => "DOUBLE",
                _ => "TEXT",
            }
        } else {
            "TEXT"
        }
    } else {
        "TEXT"
    }
}

// Generate SQL CREATE TABLE statement for the struct
fn generate_sql_create_table(fields: &Punctuated<FieldDecl, Comma>) -> String {
    let mut sql = "CREATE TABLE IF NOT EXISTS scout_entries (\n    id INTEGER PRIMARY KEY DEFAULT nextval('scout_entries_id_seq'),\n".to_string();

    // Add all other fields
    for field in fields {
        let field_name = field.name.to_string();
        let sql_type = map_to_sql_type(&field.ty);
        writeln!(sql, "    {} {},", field_name, sql_type).unwrap();
    }

    // Remove the trailing comma and newline
    sql.pop(); // Remove the newline
    sql.pop(); // Remove the comma

    // Close the statement
    sql.push_str("\n);");

    sql
}

/// Generates a struct with the specified public fields and types.
/// Also creates a constant list of (field_name, field_type, pretty_name) tuples, implements dynamic field access,
/// and generates a SQL CREATE TABLE statement.
///
/// # Example
///
/// ```
/// use struct_gen::define_struct;
///
/// define_struct!(
///     Person,
///     name: String => "Name",
///     age: u32 => "Age",
///     is_active: bool => "Active"
/// );
/// ```
#[proc_macro]
pub fn define_struct(input: TokenStream) -> TokenStream {
    // Parse the input tokens
    let input = parse_macro_input!(input as StructInput);

    // Get the struct name
    let struct_name = &input.name;

    // Create a constant name based on the struct name (e.g., Person -> PERSON_FIELDS)
    let const_name = Ident::new(
        &format!("{}_FIELDS", struct_name.to_string().to_uppercase()),
        struct_name.span(),
    );

    // Create a constant name for the pretty names
    let pretty_const_name = Ident::new(
        &format!("{}_PRETTY_NAMES", struct_name.to_string().to_uppercase()),
        struct_name.span(),
    );

    // Generate the SQL CREATE TABLE statement
    let sql_create_table = generate_sql_create_table(&input.fields);

    // Create a constant name for the SQL statement
    let sql_const_name = Ident::new("CREATE_TABLE_SQL", struct_name.span());

    // Generate row mapper function name
    let row_mapper_fn_name = format_ident!("map_datapoint");

    // Generate the field definitions with explicit "pub" visibility
    let fields = input.fields.iter().map(|field| {
        let name = &field.name;
        let ty = &field.ty;
        quote! { pub #name: #ty }
    });

    // Generate the field definitions for the InsertDataArgs, which replaces bool with Option<String>
    let insert_data_args_fields = input.fields.iter().map(|field| {
        let name = &field.name;
        let ty = &field.ty;
        if let Type::Path(TypePath { path, .. }) = ty
            && let Some(segment) = path.segments.last()
            && segment.ident == "bool"
        {
            quote! { pub #name: Option<String> }
        } else {
            quote! { pub #name: #ty }
        }
    });

    let insert_data_args_map = input.fields.iter().map(|field| {
        let name = &field.name;
        let ty = &field.ty;
        if let Type::Path(TypePath { path, .. }) = ty
            && let Some(segment) = path.segments.last()
            && segment.ident == "bool"
        {
            quote! { #name: extract_checkbox(self.#name) }
        } else {
            quote! { #name: self.#name }
        }
    });

    let insert_data_args_map_fn = quote! {
        pub fn map_insert_data_args(self) -> #struct_name {
            #[inline]
            #[allow(dead_code)]
            fn extract_checkbox(value: Option<String>) -> bool {
                value.map(|x| x == "on").unwrap_or(false)
            }
            #struct_name {
                #(#insert_data_args_map),*
            }
        }
    };

    // Get field names and types for the constant
    let field_name_types = input.fields.iter().map(|field| {
        let name = field.name.to_string();
        let type_variant =
            get_data_type_variant(&field.ty).unwrap_or(Ident::new("Unknown", field.name.span()));

        quote! { (#name, DataTypeName::#type_variant) }
    });

    let field_names = input.fields.iter().map(|field| {
        let name = field.name.to_string();

        quote! { #name }
    });

    // Generate field names and pretty names
    let field_pretty_names = input.fields.iter().map(|field| {
        let name = field.name.to_string();
        let pretty_name = match &field.pretty_name {
            Some(lit) => lit.value(),
            None => name.clone(), // Default to the field name if pretty name not provided
        };

        quote! { (#name, #pretty_name) }
    });

    // Generate row mapping field assignments (index-based)
    let row_field_assignments = input.fields.iter().enumerate().map(|(i, field)| {
        let name = &field.name;
        let index = i + 1; // Start from index 1 (assuming id is at 0)
        quote! { #name: row.get(#index)? }
    });

    let field_count = input.fields.len();

    // Generate to_sql field references
    let to_sql_field_refs = input.fields.iter().map(|field| {
        let name = &field.name;
        quote! { let #name: &dyn ToSql = &self.#name; }
    });

    // Generate to_sql array elements
    let to_sql_vector_elements = input.fields.iter().map(|field| {
        let name = &field.name;
        quote! { #name }
    });

    // Generate the row mapper function
    let row_mapper_fn = quote! {
        /// Maps a database row to a new struct instance.
        ///
        /// This function expects the row columns to match the struct fields in order.
        /// The first column (index 0) is assumed to be the ID and is not included in the struct.
        #[cfg(feature = "ssr")]
        pub fn #row_mapper_fn_name(row: &duckdb::Row<'_>) -> duckdb::Result<#struct_name> {
            Ok(#struct_name {
                #(#row_field_assignments),*
            })
        }
    };

    // Generate match arms for the get_field method
    let get_field_arms = input.fields.iter().map(|field| {
        let name = &field.name;
        let name_str = name.to_string();
        let type_variant =
            get_data_type_variant(&field.ty).unwrap_or(Ident::new("Unknown", name.span()));

        match type_variant.to_string().as_str() {
            "U16" => quote! { #name_str => Some(DataType::U16(self.#name)) },
            "U32" => quote! { #name_str => Some(DataType::U32(self.#name)) },
            "U64" => quote! { #name_str => Some(DataType::U64(self.#name)) },
            "I16" => quote! { #name_str => Some(DataType::I16(self.#name)) },
            "I32" => quote! { #name_str => Some(DataType::I32(self.#name)) },
            "I64" => quote! { #name_str => Some(DataType::I64(self.#name)) },
            "String" => quote! { #name_str => Some(DataType::String(self.#name.clone())) },
            "Bool" => quote! { #name_str => Some(DataType::Bool(self.#name)) },
            "Float" => quote! { #name_str => Some(DataType::Float(self.#name)) },
            _ => quote! { #name_str => None },
        }
    });

    // Generate match arms for the set_field method
    let set_field_arms = input.fields.iter().map(|field| {
        let name = &field.name;
        let name_str = name.to_string();
        let type_variant =
            get_data_type_variant(&field.ty).unwrap_or(Ident::new("Unknown", name.span()));

        match type_variant.to_string().as_str() {
            "U16" => quote! {
                #name_str => {
                    if let DataType::U16(value) = value {
                        self.#name = value;
                        true
                    } else { false }
                }
            },
            "U32" => quote! {
                #name_str => {
                    if let DataType::U32(value) = value {
                        self.#name = value;
                        true
                    } else { false }
                }
            },
            "U64" => quote! {
                #name_str => {
                    if let DataType::U64(value) = value {
                        self.#name = value;
                        true
                    } else { false }
                }
            },
            "I16" => quote! {
                #name_str => {
                    if let DataType::I16(value) = value {
                        self.#name = value;
                        true
                    } else { false }
                }
            },
            "I32" => quote! {
                #name_str => {
                    if let DataType::I32(value) = value {
                        self.#name = value;
                        true
                    } else { false }
                }
            },
            "I64" => quote! {
                #name_str => {
                    if let DataType::I64(value) = value {
                        self.#name = value;
                        true
                    } else { false }
                }
            },
            "String" => quote! {
                #name_str => {
                    if let DataType::String(value) = value {
                        self.#name = value;
                        true
                    } else { false }
                }
            },
            "Bool" => quote! {
                #name_str => {
                    if let DataType::Bool(value) = value {
                        self.#name = value;
                        true
                    } else { false }
                }
            },
            "Float" => quote! {
                #name_str => {
                    if let DataType::Float(value) = value {
                        self.#name = value;
                        true
                    } else { false }
                }
            },
            _ => quote! { #name_str => false },
        }
    });

    // Generate implementation
    let output = quote! {
        // Define the struct with public fields
        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
        pub struct #struct_name {
            #(#fields),*
        }

        #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
        pub struct InsertDataArgs {
            #(#insert_data_args_fields),*
        }

        impl InsertDataArgs {
            #insert_data_args_map_fn
        }

        // Define a constant with field name and type pairs
        pub const #const_name: &[(&str, DataTypeName)] = &[#(#field_name_types),*];

        // Define a constant with field name and pretty name pairs
        pub const #pretty_const_name: &[(&str, &str)] = &[#(#field_pretty_names),*];

        // Define a constant with the SQL CREATE TABLE statement
        pub const #sql_const_name: &str = #sql_create_table;

        // Implement dynamic field access
        impl #struct_name {
            /// Get a field value dynamically by field name
            pub fn get_field(&self, field_name: &str) -> Option<DataType> {
                match field_name {
                    #(#get_field_arms,)*
                    _ => None,
                }
            }

            /// Set a field value dynamically by field name
            pub fn set_field(&mut self, field_name: &str, value: DataType) -> bool {
                match field_name {
                    #(#set_field_arms,)*
                    _ => false,
                }
            }

            /// Get the field metadata (name, type pairs)
            pub fn field_metadata() -> &'static [(&'static str, DataTypeName)] {
                #const_name
            }

            /// Get the field pretty names (name, pretty_name pairs)
            pub fn field_pretty_names() -> &'static [(&'static str, &'static str)] {
                #pretty_const_name
            }

            /// Get just the field names
            pub fn field_names() -> &'static [&'static str] {
                &[#(#field_names,)*]
            }

            /// Get pretty name for a field
            pub fn get_pretty_name(field_name: &str) -> Option<&'static str> {
                #pretty_const_name.iter()
                    .find(|(name, _)| *name == field_name)
                    .map(|(_, pretty)| *pretty)
            }

            /// Get the type of a field by name
            pub fn get_field_type(field_name: &str) -> Option<DataTypeName> {
                #const_name.iter()
                    .find(|(name, _)| *name == field_name)
                    .map(|(_, typ)| *typ)
            }

            /// Get the SQL CREATE TABLE statement for this struct
            pub fn get_create_table_sql() -> &'static str {
                #sql_const_name
            }

            /// Convert the struct to a vector of SQL parameters
            #[cfg(feature = "ssr")]
            pub fn to_sql(&self) -> [&dyn duckdb::ToSql; #field_count] {
                // Create references to each field as ToSql trait objects
                #(#to_sql_field_refs)*

                // Return an array of all field references
                [
                    #(#to_sql_vector_elements),*
                ]
            }

            // Generate the row mapper function
            #row_mapper_fn
        }
    };

    // Return the generated code
    output.into()
}

/// Defines reduced columns for a specific struct.
/// This must be called after the struct has been defined with `define_struct!`.
///
/// # Example
///
/// ```
/// use struct_gen::{define_struct, define_reduced_columns};
///
/// define_struct!(
///     DataPoint,
///     name: String => "Name",
///     age: u32 => "Age"
/// );
///
/// define_reduced_columns!(DataPoint,
///     "Name" => |s| s.name.clone(),
///     "Age in Months" => |s| (s.age * 12).to_string()
/// );
/// ```
#[proc_macro]
pub fn define_reduced_columns(input: TokenStream) -> TokenStream {
    // Parse the struct name and reduced column definitions
    let parsed = parse_macro_input!(input as ReducedColumnsInput);
    let struct_name = parsed.struct_name;

    // Generate column name literals and closures
    let column_names = parsed.columns.iter().map(|col| col.name.value());
    let column_closures = parsed.columns.iter().map(|col| {
        let expr = &col.expr;
        let column_name = col.name.value();
        quote! {
            #column_name => {
                {
                    let s = self;
                    #expr
                }
            }
        }
    });

    // Generate the implementation
    let output = quote! {
        impl #struct_name {

            /// Get all reduced column values
            pub fn get_reduced_columns(&self) -> Vec<(&'static str, String)> {
                Self::reduced_column_names()
                    .into_iter()
                    .filter_map(|name| {
                        self.get_reduced_column(name).map(|value| (*name, value))
                    })
                    .collect()
            }

            /// Get the reduced column names
            pub const fn reduced_column_names() -> &'static [&'static str] {
                &[#(#column_names),*]
            }

            /// Get the value for a reduced column
            pub fn get_reduced_column(&self, column_name: &str) -> Option<String> {
                Some(match column_name {
                    #(#column_closures,)*
                    _ => return None,
                })
            }
        }
    };

    output.into()
}

// Define structures for parsing reduced columns input
struct ReducedColumnsInput {
    struct_name: Ident,
    columns: Vec<ReducedColumnDef>,
}

struct ReducedColumnDef {
    name: LitStr,
    expr: Expr,
}

impl Parse for ReducedColumnsInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let struct_name = input.parse()?;
        input.parse::<Token![,]>()?;

        let mut columns = Vec::new();

        while !input.is_empty() {
            // Parse column name (string literal)
            let name: LitStr = input.parse()?;

            // Parse arrow
            input.parse::<Token![=>]>()?;

            // Parse expression
            let expr = input.parse()?;

            columns.push(ReducedColumnDef { name, expr });

            // Skip comma if present
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            } else {
                break;
            }
        }

        Ok(ReducedColumnsInput {
            struct_name,
            columns,
        })
    }
}

#[proc_macro]
pub fn define_team_data(input: TokenStream) -> TokenStream {
    // Parse the struct name and reduced column definitions
    let parsed = parse_macro_input!(input as TeamDataInput);
    let struct_name = parsed.struct_name;

    let team_data_closures = parsed.columns.iter().map(|col| {
        let expr = &col.expr;
        let column_name = col.name.value();
        quote! { leptos::html::p().child(format!("{}: {}", #column_name, #expr)) }
    });

    // Generate the implementation
    let output = quote! {
        impl #struct_name {
            pub fn view_team_data(v: &[#struct_name]) -> leptos::prelude::AnyView {
                use leptos::prelude::*;
                view!{ <div class="team-data">{(vec![#(#team_data_closures),*]).into_any()}</div> }.into_any()
            }
        }
    };

    output.into()
}

// Define structures for parsing reduced columns input
struct TeamDataInput {
    struct_name: Ident,
    columns: Vec<TeamDataDef>,
}

struct TeamDataDef {
    name: LitStr,
    expr: Expr,
}

impl Parse for TeamDataInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let struct_name = input.parse()?;
        input.parse::<Token![,]>()?;

        let mut columns = Vec::new();

        while !input.is_empty() {
            // Parse column name (string literal)
            let name: LitStr = input.parse()?;

            // Parse arrow
            input.parse::<Token![=>]>()?;

            // Parse expression
            let expr = input.parse()?;

            columns.push(TeamDataDef { name, expr });

            // Skip comma if present
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            } else {
                break;
            }
        }

        Ok(TeamDataInput {
            struct_name,
            columns,
        })
    }
}
