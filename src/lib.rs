pub mod forms;

use wasm_bindgen::prelude::*;
pub use forms::generate_form;  // Import the function from forms module

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[wasm_bindgen]
pub fn create_html_form(json: &str) -> String {
    generate_form(json).expect("Error generating form")
}
