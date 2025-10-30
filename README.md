# TOON (Token-Oriented Object Notation) - Rust

[![Crates.io](https://img.shields.io/crates/v/toon)](https://crates.io/crates/toon)

A Rust implementation of Token-Oriented Object Notation (TOON), a compact, human-readable format designed for passing structured data to Large Language Models with significantly reduced token usage.

This is a Rust port of the original JavaScript/TypeScript implementation: [@byjohann/toon](https://github.com/johannschopplich/toon).

## Why TOON?

LLM tokens cost money, and standard JSON is verbose. TOON conveys the same information with **30-60% fewer tokens** than JSON:

**JSON** (257 tokens):

```json
{
  "users": [
    { "id": 1, "name": "Alice", "role": "admin" },
    { "id": 2, "name": "Bob", "role": "user" }
  ]
}
```

**TOON** (166 tokens):

```
users[2]{id,name,role}:
  1,Alice,admin
  2,Bob,user
```

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
toon = "0.1.0"
serde_json = "1.0"
```

## Quick Start

```rust
use toon::encode;
use serde_json::json;

fn main() {
    let data = json!({
        "user": {
            "id": 123,
            "name": "Ada",
            "tags": ["reading", "gaming"],
            "active": true
        }
    });

    let toon_output = encode(&data, None);
    println!("{}", toon_output);
}
```

Output:

```
user:
  active: true
  id: 123
  name: Ada
  tags[2]: reading,gaming
```

## Features

- 💸 **Token-efficient:** 30–60% fewer tokens than JSON
- 🍱 **Minimal syntax:** removes redundant punctuation
- 📐 **Indentation-based structure:** like YAML
- 🧺 **Tabular arrays:** declare keys once, stream rows
- 🦀 **Type-safe:** built with Rust's strong type system

## Examples

### Objects

```rust
use toon::encode;
use serde_json::json;

let data = json!({
    "id": 123,
    "name": "Ada",
    "active": true
});

println!("{}", encode(&data, None));
```

Output:

```
active: true
id: 123
name: Ada
```

### Arrays of Objects (Tabular Format)

```rust
let data = json!({
    "items": [
        { "sku": "A1", "qty": 2, "price": 9.99 },
        { "sku": "B2", "qty": 1, "price": 14.5 }
    ]
});

println!("{}", encode(&data, None));
```

Output:

```
items[2]{price,qty,sku}:
  9.99,2,A1
  14.5,1,B2
```

### Custom Delimiters

Use tab or pipe delimiters for even more token savings:

```rust
use toon::{encode, EncodeOptions, Delimiter};

let data = json!({
    "tags": ["reading", "gaming", "coding"]
});

let mut options = EncodeOptions::default();
options.delimiter = Delimiter::Tab;

println!("{}", encode(&data, Some(options)));
```

Output:

```
tags[3	]: reading	gaming	coding
```

### Length Markers

Add a `#` prefix to array lengths for clarity:

```rust
let mut options = EncodeOptions::default();
options.length_marker = Some('#');

let data = json!({"tags": ["a", "b", "c"]});
println!("{}", encode(&data, Some(options)));
```

Output:

```
tags[#3]: a,b,c
```

## API

### `encode(value: &serde_json::Value, options: Option<EncodeOptions>) -> String`

Converts a `serde_json::Value` to TOON format.

**Parameters:**

- `value`: A reference to a `serde_json::Value` to encode
- `options`: Optional encoding options (defaults to standard TOON format)

**Returns:** A `String` containing the TOON-formatted output

### `EncodeOptions`

```rust
pub struct EncodeOptions {
    pub indent: usize,                  // Spaces per indentation level (default: 2)
    pub delimiter: Delimiter,            // Delimiter for arrays (default: Comma)
    pub length_marker: Option<char>,     // Optional length prefix (default: None)
}
```

### `Delimiter`

```rust
pub enum Delimiter {
    Comma,  // ,
    Tab,    // \t
    Pipe,   // |
}
```

## Format Overview

- **Objects**: `key: value` with 2-space indentation for nesting
- **Primitive arrays**: Inline with count, e.g., `tags[3]: a,b,c`
- **Arrays of objects**: Tabular format with header, e.g., `items[2]{id,name}:`
- **Mixed arrays**: List format with `- ` prefix
- **Quoting**: Only when necessary (special chars, structural ambiguity)

## Python Bindings

Python bindings are available in the `pybinding/` directory.

### Installation

```bash
pip install toon-pyrs
```

Or build from source:

```bash
cd pybinding
pip install maturin
maturin build --release
pip install target/wheels/*.whl
```

### Python Usage

```python
import json
import toon

data = {
    "users": [
        {"id": 1, "name": "Alice", "role": "admin"},
        {"id": 2, "name": "Bob", "role": "user"}
    ]
}

result = toon.encode(json.dumps(data))
print(result)
```

Output:

```
users[2]{id,name,role}:
  1,Alice,admin
  2,Bob,user
```

### Publishing Python Package

```bash
cd pybinding
maturin build --release
pip install twine
twine upload target/wheels/*
```

## License

MIT License © 2025

## See Also

Original JS/TS implementation: [@byjohann/toon](https://github.com/johannschopplich/toon)
