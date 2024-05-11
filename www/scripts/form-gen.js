
import init, { create_html_form } from '../../pkg/form_builder.js';


// Function to load the WASM module and generate the form
export async function loadWasm() {
    await init();
    const json = `
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
                    "type": "input",
                    "id": "user_email",
                    "label": "Email",
                    "input_type": "email",
                    "placeholder": "Enter your email",
                    "required": true
                },
                {
                    "type": "button",
                    "id": "submit_button",
                    "value": "Submit"
                }
            ]
        }`;

        try {
            const formHtml = create_html_form(json);
            document.getElementById('form_container').innerHTML = formHtml;
        } catch (error) {
            console.error("Failed to generate form:", error);
            document.getElementById('form_container').textContent = 'Failed to load form.';
        }
}

// Function to submit the form
export function submitForm() {
    console.log('Form submitted');
    //event.preventDefault();
    const form = document.getElementById('myForm');
    //const form = event.target;
    let data = {};
    // Assuming all inputs are to be collected
    Array.from(form.elements).forEach(element => {
        console.log(element);
        if (element.id && element.type !== 'submit') {
            data[element.id] = element.value;
        }
    });
    console.log("Form Data:", JSON.stringify(data, null, 2));
    alert(JSON.stringify(data, null, 2));
}

// Expose the functions to the window object
window.submitForm = submitForm;
window.loadWasm = loadWasm;


