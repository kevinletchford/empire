mod utils;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use js_sys;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Empire {
    store: HashMap<String, String>,
}

#[wasm_bindgen]
impl Empire {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Empire {
        Empire {
            store: HashMap::new(),
        }
    }

    #[wasm_bindgen]
    pub fn get(&self, key: &str) -> Option<String> {
        self.store.get(key).cloned()
    }

    #[wasm_bindgen]
    pub fn set(&mut self, key: &str, value: &str) {
        self.store.insert(key.to_string(), value.to_string());
    }

    #[wasm_bindgen]
    pub fn remove(&mut self, key: &str) -> bool {
        self.store.remove(key).is_some()
    }

    #[wasm_bindgen(js_name = getAll)]
    pub fn get_all(&self) -> Vec<JsValue> {
        let mut items = Vec::new();

        for (key, value) in &self.store {
            let item = js_sys::Object::new();
            js_sys::Reflect::set(&item, &JsValue::from_str("key"), &JsValue::from_str(key));
            js_sys::Reflect::set(&item, &JsValue::from_str("value"), &JsValue::from_str(value));
            items.push(item.into());
        }

        items
    }
}