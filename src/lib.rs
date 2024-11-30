//! Provides a way to alias mutliple derives as one:
//! ```rust
//! use derive_alias::derive_alias;
//!
//! // Generates a macro (`derive_cmp`) that will attach the listed derives to a given item
//! derive_alias! {
//!     derive_cmp => #[derive(Eq, PartialEq, Ord, PartialOrd)]
//! }
//!
//! // Attach the derives to `Foo`
//! derive_cmp! { struct Foo; }
//! ```
//!
//! You can create multiple aliases at a time:
//! ```rust
//! # use derive_alias::derive_alias;
//!
//! derive_alias! {
//!     derive_cmp => #[derive(Eq, PartialEq, Ord, PartialOrd)],
//!     derive_other => #[derive(Copy, Clone)]
//! }
//!
//! derive_cmp! { struct Foo; }
//! derive_other! { struct Bar; }
//! ```

/// Refer to the [crate level documentation](crate) for details.
#[macro_export]
macro_rules! derive_alias {
    ($($name:ident => #[derive($($derive:ident),*)] $(,)?)*) => {
        $(
            macro_rules! $name {
                ($i:item) => {
                    #[derive($($derive),*)]
                    $i
                }
            }
        )*
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    derive_alias! {
        derive_one => #[derive(Copy, Clone, Debug)],
        derive_two => #[derive(Eq, PartialEq, Ord, PartialOrd)]
    }

    derive_one! {
        struct Foo;
    }

    derive_two! {
        struct Bar;
    }

    #[test]
    fn a_test() {
        assert_impl_one(Foo);
        assert_impl_two(Bar);
    }

    fn assert_impl_one<T>(_: T)
    where
        T: Copy + Clone + Debug,
    {
    }

    fn assert_impl_two<T>(_: T)
    where
        T: Eq + PartialEq + Ord + PartialOrd,
    {
    }

    #[allow(unused_macros)]
    mod test_trailing_comma {
        derive_alias! {
            foo => #[derive()]
            bar => #[derive()]
            baz => #[derive()]
        }

        derive_alias! {
            foo => #[derive()],
            bar => #[derive()],
            baz => #[derive()],
        }
    }
}
