#![allow(clippy::missing_docs_in_private_items)]

use dotenv::dotenv;
use frozen_collections::emit::{CollectionEmitter, CollectionEntry};
use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write as _;
use std::path::Path;
use syn::{parse_quote, Expr, ExprLit, Lit};
use tbaapi::apis::configuration::{ApiKey, Configuration};
use tbaapi::apis::default_api::get_search_index;

fn main() -> Result<(), Box<dyn core::error::Error>> {
    let out_dir = env::var_os("OUT_DIR").ok_or("Missing OUT_DIR env var")?;
    let dest_path = Path::new(&out_dir).join("generated.rs");
    let mut file = BufWriter::new(File::create(dest_path)?);

    dotenv()?;
    let config = Configuration {
        api_key: Some(ApiKey {
            prefix: None,
            key: env::var("TBA_API_KEY").expect("TBA_API_KEY must be set"),
        }),
        ..Configuration::default()
    };

    let rt = tokio::runtime::Runtime::new()?;
    let res = rt.block_on(async { get_search_index(&config).await })?;

    fn number_to_expr(n: u32) -> Expr {
        Expr::Lit(ExprLit {
            attrs: vec![],
            lit: Lit::Int(syn::LitInt::new(
                &n.to_string(),
                proc_macro2::Span::call_site(),
            )),
        })
    }
    fn string_to_expr(s: &str) -> Expr {
        Expr::Lit(ExprLit {
            attrs: vec![],
            lit: Lit::Str(syn::LitStr::new(s, proc_macro2::Span::call_site())),
        })
    }

    let entries: Vec<CollectionEntry<u32>> = res
        .teams
        .into_iter()
        .filter_map(|team| match team.key.trim_start_matches("frc").parse() {
            Ok(val) => Some(CollectionEntry::map_entry(
                val,
                number_to_expr(val),
                string_to_expr(&team.nickname),
            )),
            Err(_) => None,
        })
        .collect();

    let map = CollectionEmitter::new(&parse_quote! { u32 })
        .value_type(&parse_quote! { &'static str })
        .symbol_name("TEAM_NAMES")
        .static_instance(true)
        .const_keys(true)
        .const_values(true)
        .visibility(syn::Visibility::Public(syn::Token![pub](
            proc_macro2::Span::call_site(),
        )))
        .emit_scalar_collection(entries)?;
    let comment = "/// Global static variable to store team names, mapping team numbers to team
/// names. Only available when the `ssr` feature is enabled.";
    _ = writeln!(file, "{comment}\n{map}");

    println!("cargo::rerun-if-changed=build.rs");

    Ok(())
}
