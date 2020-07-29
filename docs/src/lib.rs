#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use wasm_bindgen::prelude::*;
use validity::*;

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    val.set_inner_html("
        <form onsubmit=\"myFunction()\">
            Enter name: <input type=\"text\">
            <input type=\"submit\">
        </form>
    ");

    body.append_child(&val)?;

    Ok(())
}

#[wasm_bindgen]
pub fn is_valid(s: &str) -> bool {
    is_valid_from_string(s)
}