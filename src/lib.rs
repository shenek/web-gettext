extern crate cfg_if;
extern crate wasm_bindgen;
extern crate gettext;

mod utils;
pub mod wrapper;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Translator {
    wrapper: wrapper::WrappedTranslator,
}


#[wasm_bindgen]
impl Translator {

    #[wasm_bindgen]
    pub fn new(data: &[u8]) -> Result<Translator, JsValue> {
        if let Some(w) = wrapper::WrappedTranslator::new(data) {
            Ok(Translator { wrapper: w })
        } else {
            Err(JsValue::NULL)
        }
    }

    #[wasm_bindgen]
    pub fn gettext(&self, text: String) -> String {
        self.wrapper.gettext(text)
    }

    #[wasm_bindgen]
    pub fn ngettext(&self, singular: String, plural: String, count: u32) -> String {
        self.wrapper.ngettext(singular, plural, count as u64)
    }
}
