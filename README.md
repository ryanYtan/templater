# Overview
This crate defines a simple templating language in the format `%(SELECTOR)` that
can operate on any object.

## Quick Start
Say we have a `struct` definition
```rust
struct Object {
    a: i64,
    b: String,
    c: SubObject,
}

struct SubOject {
    d: Vec<u8>,
    e: i32,
}
```
