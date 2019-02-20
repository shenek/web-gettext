//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
extern crate wasm_bindgen;
extern crate web_gettext;

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;
    use wasm_bindgen::*;
    use web_gettext::{Translator};

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn wrong_mo() {
        match Translator::new(b"xxx") {
            Ok(_) => assert!(false),
            Err(x) => assert!(x == JsValue::NULL),
        }
    }
}
