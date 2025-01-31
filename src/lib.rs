use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    // Check for potential overflow
    a.checked_add(b).unwrap_or_else(|| {
        console_error!("Integer overflow occurred");
        0
    })
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    // Limit input length for safety
    if name.len() > 1000 {
        return String::from("Error: Input too long");
    }
    format!("Hello, {}!", name)
}