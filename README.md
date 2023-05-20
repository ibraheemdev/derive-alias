# Derive Alias

[![Documentation](https://img.shields.io/badge/docs-0.1.0-4d76ae?style=for-the-badge)](https://docs.rs/derive-alias/0.1.0)
[![Version](https://img.shields.io/crates/v/derive-alias?style=for-the-badge)](https://crates.io/crates/derive-alias)
[![License](https://img.shields.io/crates/l/derive-alias?style=for-the-badge)](https://crates.io/crates/derive-alias)

Provides a way to alias mutliple derives as one:
```rust
use derive_alias::derive_alias;

// Generates a macro (`derive_cmp`) that will attach the listed derives to a given item
derive_alias! {
    derive_cmp => #[derive(Eq, PartialEq, Ord, PartialOrd)]
}

// Attach the derives to `Foo`
derive_cmp! { struct Foo; }
```

You can create multiple aliases at a time:
```rust
use derive_alias::derive_alias;

derive_alias! {
    derive_cmp => #[derive(Eq, PartialEq, Ord, PartialOrd)],
    derive_other => #[derive(Copy, Clone)]
}

derive_cmp! { struct Foo; }
derive_other! { struct Bar; }
```
