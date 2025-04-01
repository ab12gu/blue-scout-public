#[cfg(feature = "hydrate")]
mod csr {
    use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
    use web_sys::HtmlTableElement;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = window, js_name = TableFilter)]
        pub type TableFilter;

        #[wasm_bindgen(constructor)]
        pub fn new(table: &HtmlTableElement, options: &JsValue) -> TableFilter;

        #[wasm_bindgen(method)]
        pub fn init(this: &TableFilter);

        #[wasm_bindgen(method)]
        pub fn destroy(this: &TableFilter);
    }
}

#[cfg(feature = "hydrate")]
pub use csr::*;
