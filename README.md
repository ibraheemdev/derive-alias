# `derive-alias`

[<img alt="crates.io" src="https://img.shields.io/crates/v/derive-alias?style=for-the-badge" height="25">](https://crates.io/crates/derive-alias)
[<img alt="github" src="https://img.shields.io/badge/github-derive--alias-blue?style=for-the-badge" height="25">](https://github.com/ibraheemdev/derive-alias)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/derive-alias?style=for-the-badge" height="25">](https://docs.rs/derive-alias)

Provides a way to alias mutliple derives as one.

```rust
use derive_alias::derive_alias;

// Generates a macro (`derive_cmp`) that will attach the listed derives to a given item
derive_alias! {
    derive_cmp => #[derive(Eq, PartialEq, Ord, PartialOrd)]
}

// Attach the derives to `Foo`
derive_cmp! { struct Foo; }
```

You can create multiple aliases at a time.

```rust
use derive_alias::derive_alias;

derive_alias! {
    derive_cmp => #[derive(Eq, PartialEq, Ord, PartialOrd)],
    derive_other => #[derive(Copy, Clone)]
}

derive_cmp! { struct Foo; }
derive_other! { struct Bar; }
```
