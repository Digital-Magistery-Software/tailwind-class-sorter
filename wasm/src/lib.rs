use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn sort_tailwind_classes(document: &str, file_extension: &str) -> String {
    // 1. Parse document with OXC based on file_extension
    // 2. Extract class strings from the AST
    // 3. Sort the class strings
    // 4. Replace original class strings with sorted ones
    // 5. Return the modified document

    // For now, just return the input
    document.to_string()
}
