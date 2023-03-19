mod utils;

use serde::Serialize;
use wasm_bindgen::prelude::*;
use web_sys::{Headers, RequestInit};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Wasm is running");
}

#[wasm_bindgen(start)]
pub fn main() {
    greet()
}

#[wasm_bindgen]
pub fn send() {
    let window = web_sys::window().unwrap();
    let headers = Headers::new().unwrap();
    headers.append("content-type", "application/json").unwrap();

    let req = Req {
        field1: String::from("one"),
        field2: String::from("two"),
        field3: String::from("three"),
    };

    web_sys::console::log_1(&serde_wasm_bindgen::to_value(&req).unwrap());

    let ok = Closure::new(move |res| {
        web_sys::console::log_1(&res);
    });

    let _ = window
        .fetch_with_str_and_init("/api", &RequestInit::new().method("POST").headers(&headers).body(Some(&serde_wasm_bindgen::to_value(&req).unwrap())))
        .then(&ok);

    ok.forget();
}

#[derive(Serialize)]
pub struct Req {
    field1: String,
    field2: String,
    field3: String,
}
