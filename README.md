#  Hydrogen Labs

## Challenge description

Read the [challenge.md](./docs/challenge.md) file.

## Codegen Solution

### Codegen

The `generate_functions` macro is a procedural macro in Rust that generates functions based on a JSON file. This macro is defined in the [`lib.rs`](./codegen/src/lib.rs) file.

### How it works
The macro takes a string literal as input, expected to be the path to a JSON file. This JSON file should contain an object with a "methods" key that has an array of strings as its value. Each string in this array is considered a function name. The macro will generate a function for each of these function names.

The macro performs the following operations:

1. **Parse the input**: The macro parses the input as a string literal representing the path to the JSON file.
2. **Read the content of the JSON file**: The macro attempts to read the content of the JSON file. If it cannot read the file (e.g., if the file does not exist or if there is a reading error), it generates a compilation error with an appropriate error message.
3. **Parse the JSON content**: The macro attempts to parse the content of the JSON file into a `serde_json::Value`. If it cannot parse the content (e.g., if the content is not valid JSON), it generates a compilation error with an appropriate error message.
4. **Extract method names from the JSON**: The macro looks for a "methods" key in the JSON object and then tries to convert its value into an array. If it cannot find the "methods" key or if its value is not an array, it generates a compilation error with an appropriate error message.
5. **Generate Rust functions**: For each method name in the methods array, the macro generates a Rust function with that name. Each generated function simply prints a message when called.
6. **Combine the generated functions into a TokenStream**: The macro combines all the generated functions into a single `TokenStream` and returns it.

### Usage

The macro can be used as follows:

```rust
use your_macro_crate::generate_functions;

// Call the macro with the path to your JSON file
generate_functions!("path/to/your/json/file.json");

fn main() {
    // Call the generated functions
    fun1();
    fun2();
    fun3();
}
```

### Dependencies

The macro depends on the following crates:
- `proc-macro2`: A library for representing Rust code as a sequence of tokens.
- `quote`: A library for turning Rust syntax tree data structures into tokens of source code.
- `syn`: A library for parsing Rust code into syntax tree data structures.
- `serde_json`: A library for parsing JSON into Rust data structures.
