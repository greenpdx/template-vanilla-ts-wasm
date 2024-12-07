mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm!");
}

#[wasm_bindgen]
pub fn increment(num: JsValue) -> Result<f64, JsError>  {
  match num.as_f64() {
    Some(n) => Ok(n + 1.0),
    None => Err(JsError::new("Invalid Parameter"))
  }
}
