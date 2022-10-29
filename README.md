# enforce-visibility

Require that a struct and all its fields are public.
```rust
use enforce_visibility::public;

#[public]
pub struct Foo {
    pub bar: usize
}
```

```rust,compile_fail
# use enforce_visibility::public;

#[public]
pub struct Foo {
    bar: usize
}
```

```
error: field must be public: try adding `pub` before the declaration
  --> foo.rs:5:5
   |
 5 |     bar: usize,
   |     ^^^^^^^^^^^
```
