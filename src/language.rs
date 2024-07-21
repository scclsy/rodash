use js_sys::ArrayBuffer;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn cast_array(value: JsValue) -> Result<js_sys::Array, JsValue> {
    // if value.is_array() { Ok(js_sys::Array::from(&value)) }
    // else {
    //     let array = js_sys::Array::new();
    //     array.push(&value);
    //     Ok(array)
    // }
    todo!()
}