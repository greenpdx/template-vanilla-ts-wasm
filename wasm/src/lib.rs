use wasm_bindgen::prelude::*;
use web_sys::console;


mod utils;
mod fetch;
//mod webgl;
mod threed;

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
    Some(n) => {
      console::log_1(&num);
      Ok(n + 1.0)
    },
    None => Err(JsError::new("Invalid Parameter"))
  }
}

#[wasm_bindgen]
pub fn add_dom() -> Result<(), JsError> {
  let window = match web_sys::window() {
    None => return Err(JsError::new("No Window")),
    Some(w) => w
  };
  let document = match window.document() {
    None => return Err(JsError::new("No Document")),
    Some(d) => d
  };
  let val = match document.get_element_by_id("wasmdom") {
    None => return Err(JsError::new("No Id")),
    Some(v) => v
  };
  let ele = match document.create_element("p") {
    Err(e) => return Err(JsError::new(&e.as_string().unwrap())),
    Ok(e) => e
  };
  ele.set_text_content(Some("This is from Rust"));
  match val.append_child(&ele) {
    Err(_e) => return Err(JsError::new("No Append")),
    Ok(n) => n
  };
  Ok(())
}
