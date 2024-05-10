# WASM dynamic forms

needs:
cargo install wasm-pack

usage:
wasm-pack build --target web
python3 -m http.server --directory www 8080


Directory Structure
´´´
form_builder/
│
├── src/                     # Rust source files
│   ├── lib.rs               # Main library file, entry point for the WASM module
│   ├── forms.rs             # Handles form generation from JSON
│   ├── db.rs                # Database interaction logic
│   ├── api.rs               # Backend API interactions
│   └── utils.rs             # Utility functions and common helpers
│
├── www/                    # Web assets and HTML files
│   ├── index.html          # Main HTML file for the application
│   ├── styles/             # CSS files
│   │   └── main.css
│   ├── scripts/            # JavaScript files
│   │   ├── app.js          # Main JavaScript interactions
│   │   └── wasm_loader.js  # Script to load and run WASM
│   └── assets/             # Images and other assets
│
├── pkg/                    # Generated package from wasm-pack
│
├── tests/                  # Test modules for the Rust code
│   ├── forms_tests.rs      # Tests for form handling
│   └── db_tests.rs         # Tests for database functionality
│
├── target/                 # Compiled files and build scripts
├── Cargo.toml              # Rust package manifest
└── Cargo.lock              # Dependency lock file (automatically generated)

´´´