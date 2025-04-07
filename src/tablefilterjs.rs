//! This module provides a Rust interface to the `TableFilter` JavaScript
//! library.
//!
//! It allows you to create, initialize, and destroy `TableFilter` instances
//! to add filtering functionality to HTML tables in a web application.
//!
//! The module is only available when the `hydrate` feature is enabled,
//! as it relies on the `wasm_bindgen` crate for interoperability with
//! JavaScript.

#[cfg(feature = "hydrate")]
mod csr {
    //! Ensure this module is only compiled when the `hydrate` feature is
    //! enabled.
    use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
    use web_sys::HtmlTableElement;

    /// Exposes the tablefilter.js library to rust.
    #[wasm_bindgen]
    extern "C" {
        /// Represents the TableFilter class in JavaScript.
        #[wasm_bindgen(js_namespace = window, js_name = TableFilter)]
        pub type TableFilter;

        /// Creates a new TableFilter instance.
        ///
        /// # Arguments
        ///
        /// * `table` - The HTML table element to apply the filters to.
        /// * `options` - A JavaScript object containing configuration options
        ///   for the filters.
        #[wasm_bindgen(constructor)]
        pub fn new(table: &HtmlTableElement, options: &JsValue) -> TableFilter;

        /// Initializes the TableFilter instance.
        ///
        /// This method must be called after creating a new instance to activate
        /// the filter.
        #[wasm_bindgen(method)]
        pub fn init(this: &TableFilter);

        /// Destroys the TableFilter instance, removing all traces of it.
        #[wasm_bindgen(method)]
        pub fn destroy(this: &TableFilter);
    }
}

#[cfg(feature = "hydrate")]
pub use csr::*;
