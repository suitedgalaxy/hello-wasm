use wasm_bindgen::prelude::*;
use web_sys;

#[wasm_bindgen]
extern "C" {
	pub fn alert(s: &str);
}

#[wasm_bindgen(start)]
pub unsafe fn main() -> Result<(), JsValue> {
	alert("Hello, me!");
	let window = web_sys::window().expect("no global `window` exits");
	let document = window.document().expect("should have document on window");
	let body = document.body().expect("should have body");

	let val = document.create_element("p")?;
	val.set_inner_html("Hello from Rust!");

	body.append_child(&val)?;
	Ok(())
}