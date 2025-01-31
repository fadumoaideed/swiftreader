# Creating and Publishing a Rust WebAssembly Library

This guide walks through creating a Rust library that compiles to WebAssembly (Wasm) and can be used in multiple programming languages.

## Project Setup

1. Install Rust and Cargo if you haven't already:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Create a new Rust library:

```bash
cargo new my_rust_lib --lib
cd my_rust_lib
```

3. Configure `Cargo.toml`:

```toml
[package]
name = "my_rust_lib"
version = "0.1.0"
edition = "2021"
description = "A WebAssembly library built with Rust"
license = "MIT"
repository = "https://github.com/your-username/my_rust_lib"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
wasm-bindgen = "0.2"
serde = { version = "1.0", features = ["derive"] }
serde-wasm-bindgen = "0.5"

[dev-dependencies]
wasm-bindgen-test = "0.3"
```

4. Create your library code in `src/lib.rs`:

```rust
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

// Basic example functions
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

// Example of handling complex data types
#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Point {
    x: f64,
    y: f64,
}

#[wasm_bindgen]
impl Point {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }

    pub fn distance_from_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}
```

## Building for WebAssembly

1. Install wasm-pack:

```bash
cargo install wasm-pack
```

2. Build the library for different targets:

For web browsers (bundler):

```bash
wasm-pack build --target bundler
```

For Node.js:

```bash
wasm-pack build --target nodejs
```

For no modules (vanilla JS):

```bash
wasm-pack build --target no-modules
```

## Publishing to npm

1. Update the generated `pkg/package.json`:

```json
{
  "name": "@your-npm-username/my-rust-lib",
  "version": "0.1.0",
  "type": "module",
  "files": [
    "my_rust_lib_bg.wasm",
    "my_rust_lib.js",
    "my_rust_lib_bg.js",
    "my_rust_lib.d.ts"
  ],
  "module": "my_rust_lib.js",
  "types": "my_rust_lib.d.ts",
  "sideEffects": false,
  "description": "A WebAssembly library built with Rust",
  "repository": {
    "type": "git",
    "url": "https://github.com/your-username/my-rust-lib"
  },
  "keywords": ["wasm", "rust", "webassembly"],
  "license": "MIT"
}
```

2. Login to npm:

```bash
npm login
```

3. Publish the package:

```bash
cd pkg
npm publish --access public
```

## Using the Library

### In JavaScript/TypeScript

1. Install the package:

```bash
npm install @your-npm-username/my-rust-lib
```

2. Usage in TypeScript:

```typescript
import init, { add, greet, Point } from '@your-npm-username/my-rust-lib';

async function main() {
  // Initialize the Wasm module
  await init();

  // Use basic functions
  console.log(add(2, 3)); // Output: 5
  console.log(greet('World')); // Output: "Hello, World!"

  // Use complex types
  const point = new Point(3, 4);
  console.log(point.distance_from_origin()); // Output: 5
}

main();
```

3. Usage in vanilla JavaScript:

```html
<!doctype html>
<html>
  <head>
    <meta charset="UTF-8" />
    <title>Wasm Example</title>
  </head>
  <body>
    <script type="module">
      import init, {
        add,
        greet,
      } from './node_modules/@your-npm-username/my-rust-lib/my_rust_lib.js';

      async function run() {
        await init();
        console.log(add(2, 3));
        console.log(greet('World'));
      }

      run();
    </script>
  </body>
</html>
```

### In Python (using WASM in Node.js)

1. Install required packages:

```bash
pip install wasmtime
```

2. Usage example:

```python
from wasmtime import Store, Module, Instance
import requests

# Download the wasm file from npm (you'll need to host it somewhere)
wasm_url = "https://your-cdn.com/my_rust_lib_bg.wasm"
wasm_bytes = requests.get(wasm_url).content

# Initialize the WebAssembly module
store = Store()
module = Module(store.engine, wasm_bytes)
instance = Instance(store, module, [])

# Use the functions
add = instance.exports["add"]
result = add(2, 3)
print(result)  # Output: 5
```

## Testing

1. Add tests to `src/lib.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[wasm_bindgen_test]
    fn test_greet() {
        assert_eq!(greet("World"), "Hello, World!");
    }
}
```

2. Run tests:

```bash
# Run native Rust tests
cargo test

# Run Wasm tests in Chrome
wasm-pack test --chrome

# Run Wasm tests in Firefox
wasm-pack test --firefox

# Run Wasm tests in Node.js
wasm-pack test --node
```

## Next Steps

1. **Documentation**:

   - Add detailed API documentation using `rustdoc`
   - Create usage examples for different languages
   - Add a CHANGELOG.md file

2. **CI/CD**:

   - Set up GitHub Actions for automated testing and publishing
   - Add version tags and releases

3. **Features**:

   - Add error handling
   - Implement async functions
   - Add more complex data structures
   - Optimize performance

4. **Distribution**:
   - Create examples for different frameworks (React, Vue, etc.)
   - Add bundler configurations (webpack, rollup, etc.)
   - Create language-specific wrappers

## Renaming Your Project

If you need to rename your project after creation, follow these steps:

1. Update the project name in `Cargo.toml`:

```toml
[package]
name = "your_new_name"  # Change this
```

2. Update the package name in `pkg/package.json`:

```json
{
  "name": "@your-npm-username/your-new-name",
  "files": [
    "your_new_name_bg.wasm",
    "your_new_name.js",
    "your_new_name_bg.js",
    "your_new_name.d.ts"
  ],
  "module": "your_new_name.js",
  "types": "your_new_name.d.ts"
}
```

3. Rebuild the project:

```bash
wasm-pack build
```

Note: After renaming, make sure to update any import statements in your JavaScript/TypeScript code to use the new package name.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Security Considerations

Before publishing your WebAssembly package to npm, consider these security best practices:

1. **Input Validation**:

   - Always validate and sanitize input parameters
   - Implement length checks for strings and arrays
   - Handle integer overflow/underflow cases
   - Consider using Rust's `checked_*` operations for arithmetic

2. **Memory Safety**:

   - Be cautious with dynamic memory allocation
   - Implement limits on memory usage
   - Clean up resources properly using Drop trait
   - Avoid exposing raw pointers to JavaScript

3. **Package Security**:

   - Use `npm audit` to check for dependencies vulnerabilities
   - Keep dependencies up to date
   - Set appropriate package permissions
   - Use version pinning for dependencies

4. **Publishing Best Practices**:

   - Enable 2FA (Two-Factor Authentication) on your npm account:
     ```bash
     npm profile enable-2fa auth-and-writes
     ```
   - Use `.npmignore` to exclude sensitive files:
     ```
     tests/
     examples/
     .gitignore
     .travis.yml
     *.log
     ```
   - Add a `prepublishOnly` script to ensure clean builds:
     ```json
     {
       "scripts": {
         "prepublishOnly": "wasm-pack build"
       }
     }
     ```

5. **Code Exposure**:

   - WebAssembly code can be reverse-engineered
   - Don't include sensitive information in your code
   - Consider using code obfuscation if necessary
   - Keep sensitive business logic on the server side

6. **Rate Limiting**:

   - Implement rate limiting for resource-intensive operations
   - Add timeouts for long-running operations
   - Consider adding memory usage limits

7. **Error Handling**:

   - Don't expose detailed error messages to the client
   - Implement proper error handling for all operations
   - Log errors appropriately without exposing sensitive information

8. **Documentation**:

   - Document security considerations for users
   - Provide clear usage guidelines
   - Include security policy (SECURITY.md)
   - Document known limitations

9. **Testing**:
   - Include security-focused tests
   - Test edge cases and error conditions
   - Implement fuzzing tests for input validation
   - Test memory usage patterns

Remember: WebAssembly runs with the same permissions as JavaScript in the browser, so follow the same security principles you would for any client-side code.
