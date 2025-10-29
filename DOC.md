# TOON Rust Implementation - Summary

This document describes the Rust port of the TOON (Token-Oriented Object Notation) library.

## Project Structure

```
toon-rs/
├── Cargo.toml           # Project configuration
├── README.md            # User-facing documentation
├── IMPLEMENTATION.md    # This file
├── src/
│   ├── lib.rs          # Public API
│   ├── types.rs        # Core type definitions
│   ├── normalize.rs    # Value normalization logic
│   ├── primitives.rs   # Primitive encoding and quoting
│   ├── writer.rs       # Indented line writer utility
│   └── encoders.rs     # Main encoding logic
└── examples/
    └── basic.rs        # Example usage
```

## Core Modules

### 1. types.rs
Defines the core data structures:
- `JsonPrimitive`: Enum for primitive types (String, Number, Boolean, Null)
- `JsonValue`: Enum for all JSON values (Primitive, Object, Array)
- `Delimiter`: Enum for delimiter types (Comma, Tab, Pipe)
- `EncodeOptions`: Configuration for encoding behavior

### 2. normalize.rs
Converts `serde_json::Value` to our internal `JsonValue` representation. Handles:
- Special number values (NaN, Infinity → null)
- Canonicalization (-0 → 0)
- Type checking utilities (is_primitive, is_array_of_objects, etc.)

### 3. primitives.rs
Implements string encoding and quoting rules:
- `encode_primitive()`: Encodes primitive values
- `encode_string_literal()`: Handles string quoting
- `encode_key()`: Encodes object keys
- `format_header()`: Generates array headers with length/field info
- Quoting rules match the JS version exactly

### 4. writer.rs
Simple utility for building indented output:
- Manages indentation levels
- Joins lines with newlines

### 5. encoders.rs
Main encoding logic with recursive traversal:
- `encode_value()`: Entry point for encoding
- `encode_object()`: Handles object encoding
- `encode_array()`: Dispatches to appropriate array encoding strategy
- `detect_tabular_header()`: Checks if array can use tabular format
- `encode_array_of_objects_as_tabular()`: Tabular format encoder
- `encode_mixed_array_as_list_items()`: List format for mixed arrays
- `encode_object_as_list_item()`: Encodes objects within lists

### 6. lib.rs
Public API:
- `encode(value, options)`: Main encoding function
- Re-exports `Delimiter` and `EncodeOptions`
- Integrates normalization and encoding steps

## Key Design Decisions

### HashMap vs Insertion Order
The original JS implementation preserves object key insertion order (ES6+ Map/Object behavior). Rust's `HashMap` does not guarantee order, so we currently sort keys alphabetically. 

**Future improvement**: Consider using `indexmap::IndexMap` to preserve insertion order while maintaining O(1) lookups.

### Regex Usage
Uses the `regex` crate for:
- Detecting numeric-like strings
- Validating unquoted key patterns

### Error Handling
Currently, the library doesn't return `Result` types as encoding should always succeed for valid `serde_json::Value` inputs. Invalid data is normalized to safe representations (e.g., NaN → null).

## Differences from JavaScript Version

1. **Key Ordering**: Keys are sorted alphabetically instead of preserving insertion order
2. **Numeric Formatting**: Rust's float formatting may differ slightly from JS in edge cases
3. **Type Safety**: Rust's type system provides compile-time guarantees that JS lacks

## Testing

Basic tests verify:
- Primitive encoding
- Simple objects
- Primitive arrays
- Empty objects

**Future work**: Port comprehensive test suite from the JS version to ensure exact compatibility.

## Performance Considerations

- Zero-copy string handling where possible
- Pre-allocated `String` builders in `LineWriter`
- Efficient regex compilation (compile once, use many times)
- No unnecessary cloning except where needed for ownership

## Dependencies

- `serde_json`: For JSON value representation and parsing
- `regex`: For pattern matching in quoting rules

## Usage Example

```rust
use toon::encode;
use serde_json::json;

let data = json!({
    "items": [
        {"sku": "A1", "qty": 2},
        {"sku": "B2", "qty": 1}
    ]
});

let output = encode(&data, None);
// Output:
// items[2]{qty,sku}:
//   2,A1
//   1,B2
```

## Future Enhancements

1. **Preserve insertion order**: Use `IndexMap` instead of `HashMap`
2. **More tests**: Port complete test suite from JS version
3. **Benchmarks**: Compare performance with JSON serialization
4. **Streaming encoder**: Support encoding large datasets without loading everything into memory
5. **Custom serialization**: Implement `serde::Serialize` trait for direct struct encoding
6. **Decoder**: Implement parsing of TOON format back to JSON

## License

MIT License © 2025
