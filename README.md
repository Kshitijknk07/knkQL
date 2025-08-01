# knkQL

`knkQL` is a lightweight, custom query language parser built from scratch in **Rust**. It transforms clean, human-readable queries into a strongly typed format for efficient processing. Designed for simplicity and developer ergonomics, `knkQL` provides a minimal yet powerful syntax for data querying.

## Features

* *   **Readable Syntax**: Intuitive query structure with clear keywords (`read`, `whom`) and field selection.
* *   **Strongly Typed Output**: Parses queries into a structured Rust `Query` type.
* *   **Pure Rust**: Built without third-party parsing libraries for performance and control.
* *   **Modular Design**: Organized project structure for scalability and maintainability.

## Supported Query Format

Currently, `knkQL` supports the `read` action to fetch data from a specified resource.

### Example Query

```bash
read user(123) {
  name
  email
}
```

### Parsed Output

```rust
Query {
    action: "read",
    whom: "user",
    id: 123,
    fields: ["name", "email"],
}
```

## Implementation Details

### 1\. Custom Query Syntax

* *   **Action**: Uses `read` to clearly indicate the query’s purpose.
* *   **Resource**: Specified via `whom(id)` (e.g., `user(123)`).
* *   **Fields**: Defined in `{}` blocks, separated by whitespace or newlines.
* *   **Ergonomics**: Avoids generic terms like `query` or `resource` for clarity.

### 2\. Parsing Logic

* *   Built with core Rust string operations.
* *   Extracts:* *   `whom`: Resource name between `read` and `(`.
*     `id`: Integer within parentheses.
*     `fields`: Values in `{}` block.
* *   No external dependencies, ensuring lightweight and reliable parsing.

### 3\. Strongly Typed Output

The parser outputs a Rust struct:

```rust
struct Query {
    action: String,
    whom: String,
    id: i32,
    fields: Vec<String>,
}
```

### 4\. Project Structure

Organized for maintainability:

```
knkql/
├── src/
│   ├── main.rs      # Entry point for input handling and parsing
│   └── query.rs     # Query struct and parsing logic
├── Cargo.toml       # Rust project configuration
```

```
Building by me ~ knk MORE TO COME :))
```

