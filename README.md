# knkQL

`knkQL` is a custom query language parser built from scratch in Rust. It accepts clean, structured, human-readable queries and parses them into a strongly typed format for further processing.

## ✅ Supported Query Format

The query engine currently supports a single operation:
read user(123) {
name
email
}


It extracts:
- `action`: `"read"`
- `whom`: `"user"`
- `id`: `123`
- `fields`: `["name", "email"]`

## 🧠 What I have Tried to Compelte So Far!!

### 1. Custom Query Design
- Chose the keyword `read` (instead of generic terms like `query`).
- Replaced generic `resource` with personalized term `whom` for clarity.
- Chose a format that is minimal, human-readable, and visually structured.

### 2. Core Parser Logic
- Extracted `whom` by splitting the string after `read` and before `(`.
- Extracted `id` by parsing the number inside `(...)`.
- Extracted `fields` by slicing inside `{}` and splitting by whitespace.
- All parts are parsed using clean string operations.

### 3. Typed Output
- Grouped all parsed values into a custom `Query` struct with:
  - `action: String`
  - `whom: String`
  - `id: i32`
  - `fields: Vec<String>`
- Returned the struct from a parser function.
- Printed final parsed result using debug output.

### 4. Codebase Organization
- Separated parsing logic into `query.rs`.
- Kept entry point clean in `main.rs`.
- Used module system to structure the project in a scalable way.

## 📁 Folder Structure
knkql/
├── src/
│   ├── main.rs        # Entry point
│   └── query.rs       # Query struct and parser logic
├── Cargo.toml         # Rust project config


## 🔄 Example Output

For input:
read user(123) {
name
email
}


The output struct is:
```rust
Query {
    action: "read",
    whom: "user",
    id: 123,
    fields: ["name", "email"],
}