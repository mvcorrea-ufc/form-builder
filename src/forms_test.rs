extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
wasm_bindgen_test_configure!(run_in_browser);

use super::*;

// Run the tests with the following command: (need firefox installed)
// wasm-pack test --headless --firefox 
// to generate binaries
// wasm-pack build --target web

/*
// Testing Form Submission handling
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test(async)]
fn test_form_submission() {
    let json = r#"
    {
        "form_elements": [
            {
                "type": "input",
                "id": "user_email",
                "label": "Email",
                "input_type": "email",
                "placeholder": "Enter your email"
            },
            {
                "type": "button",
                "id": "submit_button",
                "value": "Submit",
                "action": "submit"
            }
        ]
    }"#;

    let _form_html = generate_form(json).unwrap();

    // Simulate a form submission; check the collected data or event triggering.
    // Here you would typically interact with the JS side of the WebAssembly.
    // This may require mocking or setting up a JS environment to capture events.
}
*/

// Testing Input Validation
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test]
fn test_input_validation() {
    let json = r#"
    {
        "form_elements": [
            {
                "type": "input",
                "id": "email",
                "label": "Email",
                "input_type": "email",
                "required": true
            }
        ]
    }"#;

    let form_html = generate_form(json).unwrap();
    assert!(form_html.contains("required"), "Email input should have 'required' attribute");
}

// Testing Error Handling
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test]
fn test_error_handling() {
    let json = r#" { "form_elements": { "id": "broken" } } "#;  // Incorrect JSON
    assert!(generate_form(json).is_err(), "Should return an error for incorrect JSON format");
}

// Testing Form Creation
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test]
fn test_create_html_form() {
    let json = r#"
    {
        "form_elements": [
            {
                "type": "input",
                "id": "user_name",
                "label": "Name",
                "input_type": "text",
                "placeholder": "Enter your full name",
                "required": true
            },
            {
                "type": "button",
                "id": "submit_button",
                "value": "Submit"
            }
        ]
    }"#;

    let expected_output = "<form id='myForm' onsubmit='submitForm(); return false;'><br>\
                            <label for='user_name'>Name</label><input type='text' id='user_name' placeholder='Enter your full name' required/><br>\
                            <button type='submit' id='submit_button'>Submit</button>\
                            </form>";
    
    assert_eq!(generate_form(json).unwrap(), expected_output);
}

// Dummy test
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test]
fn test_sample_ok() {
    assert_eq!(2 + 2, 4);
}

