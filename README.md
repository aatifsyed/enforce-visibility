# xlint

Lightweight lints based on proc-macros.

### Require that a struct and all its fields are public with [`public`]
```rust
#[xlint::public]
pub struct Foo {
    pub bar: usize
}
```

```rust,compile_fail
#[xlint::public]
pub struct Foo {
    bar: usize
}
```

```
error: field must be public: try adding `pub` before the declaration
  --> foo.rs:5:5
   |
 5 |     bar: usize,
   |     ^^^^^^^
```

License: Apache 2.0 OR MIT
