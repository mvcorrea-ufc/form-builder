use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Form {
    form_elements: Vec<FormElement>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FormElement {
    #[serde(rename = "type")]
    el_type: String,
    id: String,
    label: Option<String>,
    input_type: Option<String>,
    placeholder: Option<String>,
    required: Option<bool>,
    value: Option<String>,
    //action: Option<String>,
}


#[wasm_bindgen]
pub fn generate_form(json: &str) -> Result<String, JsValue> {
    match serde_json::from_str::<Form>(json) {
        Ok(form) => {
            let mut html = String::from("<form id='myForm' onsubmit='submitForm(); return false;'><br>");

            for element in form.form_elements {
                html.push_str(&match element.el_type.as_str() {
                    "input" => format!(
                        "<label for='{id}'>{label}</label><input type='{input_type}' id='{id}' placeholder='{placeholder}' {required}/><br>",
                        id = element.id,
                        label = element.label.unwrap_or_default(),
                        input_type = element.input_type.unwrap_or_else(|| "text".to_string()),
                        placeholder = element.placeholder.unwrap_or_default(),
                        required = if element.required.unwrap_or(false) { "required" } else { "" },
                    ),
                    "button" => format!(
                        "<button type='submit' id='{id}'>{value}</button>",
                        id = element.id,
                        value = element.value.unwrap_or_default()
                    ),
                    _ => String::new(),
                });
            }

            html.push_str("</form>");
            Ok(html)
        },
        Err(e) => Err(JsValue::from_str(&format!("Error parsing JSON: {}", e)))
    }
}


#[cfg(test)]
#[path = "./forms_test.rs"]
mod forms_test;

