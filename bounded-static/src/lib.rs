#![doc(html_root_url = "https://docs.rs/bounded-static/0.5.0")]
//! Provides the [`ToBoundedStatic`] and [`IntoBoundedStatic`] traits and [ToStatic] derive macro.
//!
//! As described in the [Common Rust Lifetime Misconceptions](https://github.com/pretzelhammer/rust-blog/blob/master/posts/common-rust-lifetime-misconceptions.md#2-if-t-static-then-t-must-be-valid-for-the-entire-program):
//!
//! > `T: 'static` should be read as "`T` is bounded by a `'static` lifetime" not "`T` has a `'static` lifetime".
//!
//! The traits [`ToBoundedStatic`] and [`IntoBoundedStatic`] can be used to convert any suitable `T` and `&T` to an
//! owned `T` such that `T: 'static`.  Both traits define an associated type which is bounded by `'static` and provide
//! a method to convert to that bounded type:
//!
//! ```rust
//! pub trait ToBoundedStatic {
//!     type Static: 'static;
//!
//!     fn to_static(&self) -> Self::Static;
//! }
//!
//! pub trait IntoBoundedStatic {
//!     type Static: 'static;
//!
//!     fn into_static(self) -> Self::Static;
//! }
//! ```
//!
//! Implementations of [`ToBoundedStatic`] and [`IntoBoundedStatic`] are provided for the following `core` types:
//!
//! - [`primitive`](core::primitive) (no-op conversions)
//! - [`array`](array)
//! - [`tuple`](tuple)
//! - [`Option`](core::option::Option)
//! - [`Result`](core::result::Result)
//!
//! Additional implementations are available by enabling the following features:
//!
//! - `alloc` for common types from the `alloc` crate:
//!   - [Cow](https://doc.rust-lang.org/alloc/borrow/enum.Cow.html)
//!   - [String](https://doc.rust-lang.org/alloc/string/struct.String.html)
//!   - [Vec](https://doc.rust-lang.org/alloc/vec/struct.Vec.html)
//!   - [Box](https://doc.rust-lang.org/alloc/boxed/struct.Box.html)
//!
//! - `collections` for all collection types in the `alloc` crate:
//!   - [BinaryHeap](https://doc.rust-lang.org/alloc/collections/binary_heap/struct.BinaryHeap.html)
//!   - [BTreeMap](https://doc.rust-lang.org/alloc/collections/btree_map/struct.BTreeMap.html)
//!   - [BTreeSet](https://doc.rust-lang.org/alloc/collections/btree_set/struct.BTreeSet.html)
//!   - [LinkedList](https://doc.rust-lang.org/alloc/collections/linked_list/struct.LinkedList.html)
//!   - [VecDeque](https://doc.rust-lang.org/alloc/collections/vec_deque/struct.VecDeque.html)
//!
//! - `std` for additional types from `std`:
//!   - [HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html)
//!   - [HashSet](https://doc.rust-lang.org/std/collections/struct.HashSet.html)
//!
//! Note that `collections`, `alloc` and `std` are enabled be default.
//!
//! Additional implementations for 3rd party types are available by enabling the following features:
//!
//! - `smol_str` for [`SmolStr`](https://docs.rs/smol_str/0.1.21/smol_str/struct.SmolStr.html)
//! - `smallvec` for [`SmallVec`](https://docs.rs/smallvec/1.8.0/smallvec/struct.SmallVec.html)
//! - `smartstring` for [`SmartString`](https://docs.rs/smartstring/1.0.0/smartstring/index.html)
//!
//! # Examples
//!
//! Given a structure which can be borrow or owned and a function which requires its argument is bounded by the
//! `'static` lifetime:
//!
//! ```rust
//! # use std::borrow::Cow;
//! struct Foo<'a> {
//!     bar: Cow<'a, str>,
//!     baz: Vec<Cow<'a, str>>,
//! }
//!
//! fn ensure_static<T: 'static>(_: T) {}
//! ```
//!
//! We can implement [`ToBoundedStatic`] (and [`IntoBoundedStatic`]) for `Foo<'_>`:
//!
//! ```rust
//! # use std::borrow::Cow;
//! # use bounded_static::ToBoundedStatic;
//! struct Foo<'a> {
//!     bar: Cow<'a, str>,
//!     baz: Vec<Cow<'a, str>>,
//! }
//! impl ToBoundedStatic for Foo<'_> {
//!     type Static = Foo<'static>;
//!
//!     fn to_static(&self) -> Self::Static {
//!         Foo { bar: self.bar.to_static(), baz: self.baz.to_static() }
//!     }
//! }
//! ```
//!
//! This allows is to convert to an owned representation such that it is now bounded by `'static`:
//!
//! ```rust
//! #[test]
//! fn test() {
//!     # fn ensure_static<T: 'static>(_: T) {}
//!     let s = String::from("data");
//!     let foo = Foo { bar: Cow::from(&s), baz: vec![Cow::from(&s)] };
//!     let to_static = foo.to_static();
//!     ensure_static(to_static);
//! }
//! ```
//!
//! # Derive
//!
//! These traits may be automatically derived for any `struct` or `enum` that can be converted to a form that is
//! bounded by `'static` by using the [`ToStatic`] macro. It support all `struct` flavors (unit, named & unnamed),
//! all `enum` variant flavors (unit, named & unnamed).  It does not currently support `union`.
//!
//! To use the [`ToStatic`] macro you must enable the `derive` feature:
//!
//! ```yaml
//! bounded-static = { version = "0.5.0", features = [ "derive" ] }
//! ```
//!
//! # Examples
//!
//! ```rust
//! # use std::borrow::Cow;
//! # use std::collections::HashMap;
//! # use bounded_static::ToStatic;
//! /// Named field struct
//! #[derive(ToStatic)]
//! struct Foo<'a> {
//!     aaa: Cow<'a, str>,
//!     bbb: &'static str,
//!     ccc: Baz<'a>,
//! }
//!
//! /// Unnamed field struct
//! #[derive(ToStatic)]
//! struct Bar<'a, 'b>(u128, HashMap<Cow<'a, str>, Cow<'b, str>>);
//!
//! /// Unit struct
//! #[derive(ToStatic)]
//! struct Qux;
//!
//! #[derive(ToStatic)]
//! enum Baz<'a> {
//!     First(String, usize, Vec<Cow<'a, str>>),
//!     Second { fst: u32, snd: &'static str },
//!     Third,
//! }
//! ```
#![warn(clippy::all, clippy::pedantic, clippy::nursery, rust_2018_idioms)]
#![allow(clippy::missing_const_for_fn)]
#![forbid(unsafe_code)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

use core::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
};

#[cfg(feature = "alloc")]
use alloc::{
    borrow::{Cow, ToOwned},
    boxed::Box,
    string::String,
    vec::Vec,
};

#[cfg(feature = "collections")]
use alloc::collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque};

#[cfg(feature = "derive")]
/// Re-export for the custom derive macro `ToStatic`.
pub use bounded_static_derive::ToStatic;

/// A trait for converting `&T` to an owned `T` such that `T: 'static`.
///
/// See the module level documentation for details.
pub trait ToBoundedStatic {
    /// The target type is bounded by the `'static` lifetime.
    type Static: 'static;

    /// Convert an `&T` to an owned `T` such that `T: 'static`.
    #[must_use = "converting is often expensive and is not expected to have side effects"]
    fn to_static(&self) -> Self::Static;
}

/// A trait for converting an owned `T` into an owned `T` such that `T: 'static`.
///
/// See the module level documentation for details.
pub trait IntoBoundedStatic {
    /// The target type is bounded by the `'static` lifetime.
    type Static: 'static;

    /// Convert an owned `T` into an owned `T` such that `T: 'static`.
    #[must_use = "converting is often expensive and is not expected to have side effects"]
    fn into_static(self) -> Self::Static;
}

/// No-op [`ToBoundedStatic`] impl for converting `&'static str` to `&'static str`.
impl ToBoundedStatic for &'static str {
    type Static = &'static str;

    fn to_static(&self) -> Self::Static {
        self
    }
}

/// No-op [`IntoBoundedStatic`] impl for converting `&'static str` into `&'static str`.
impl IntoBoundedStatic for &'static str {
    type Static = &'static str;

    fn into_static(self) -> Self::Static {
        self
    }
}

/// No-op [`ToBoundedStatic`] and [`IntoBoundedStatic`] impls for primitive types.
macro_rules! make_primitive_impl {
    ($id:ident) => {
        /// No-op [`ToBoundedStatic`] impl for this primitive type.
        impl ToBoundedStatic for $id {
            type Static = $id;

            fn to_static(&self) -> Self::Static {
                *self
            }
        }
        /// No-op [`IntoBoundedStatic`] impl for this primitive type.
        impl IntoBoundedStatic for $id {
            type Static = $id;

            fn into_static(self) -> Self::Static {
                self
            }
        }
    };
}

make_primitive_impl!(bool);
make_primitive_impl!(char);
make_primitive_impl!(f32);
make_primitive_impl!(f64);
make_primitive_impl!(usize);
make_primitive_impl!(u8);
make_primitive_impl!(u16);
make_primitive_impl!(u32);
make_primitive_impl!(u64);
make_primitive_impl!(u128);
make_primitive_impl!(isize);
make_primitive_impl!(i8);
make_primitive_impl!(i16);
make_primitive_impl!(i32);
make_primitive_impl!(i64);
make_primitive_impl!(i128);
make_primitive_impl!(NonZeroUsize);
make_primitive_impl!(NonZeroU8);
make_primitive_impl!(NonZeroU16);
make_primitive_impl!(NonZeroU32);
make_primitive_impl!(NonZeroU64);
make_primitive_impl!(NonZeroU128);
make_primitive_impl!(NonZeroIsize);
make_primitive_impl!(NonZeroI8);
make_primitive_impl!(NonZeroI16);
make_primitive_impl!(NonZeroI32);
make_primitive_impl!(NonZeroI64);
make_primitive_impl!(NonZeroI128);

/// No-op [`ToBoundedStatic`] impl for unit type `()`.
impl ToBoundedStatic for () {
    type Static = ();

    fn to_static(&self) -> Self::Static {}
}

/// No-op [`IntoBoundedStatic`] impl for unit type `()`.
impl IntoBoundedStatic for () {
    type Static = ();

    fn into_static(self) -> Self::Static {}
}

/// Blanket [`ToBoundedStatic`] impl for converting `Option<T>` to `Option<T>: 'static`.
impl<T> ToBoundedStatic for Option<T>
where
    T: ToBoundedStatic,
{
    type Static = Option<T::Static>;

    fn to_static(&self) -> Self::Static {
        self.as_ref().map(ToBoundedStatic::to_static)
    }
}

/// Blanket [`IntoBoundedStatic`] impl for converting `Option<T>` into `Option<T>: 'static`.
impl<T> IntoBoundedStatic for Option<T>
where
    T: IntoBoundedStatic,
{
    type Static = Option<T::Static>;

    fn into_static(self) -> Self::Static {
        self.map(IntoBoundedStatic::into_static)
    }
}

/// Blanket [`ToBoundedStatic`] impl for converting `Result<T, E>` to `Result<T, E>: 'static`.
impl<T, E> ToBoundedStatic for Result<T, E>
where
    T: ToBoundedStatic,
    E: ToBoundedStatic,
{
    type Static = Result<T::Static, E::Static>;

    fn to_static(&self) -> Self::Static {
        match self {
            Ok(value) => Ok(value.to_static()),
            Err(err) => Err(err.to_static()),
        }
    }
}

/// Blanket [`IntoBoundedStatic`] impl for converting `Result<T, E>` into `Result<T, E>: 'static`.
impl<T, E> IntoBoundedStatic for Result<T, E>
where
    T: IntoBoundedStatic,
    E: IntoBoundedStatic,
{
    type Static = Result<T::Static, E::Static>;

    fn into_static(self) -> Self::Static {
        match self {
            Ok(value) => Ok(value.into_static()),
            Err(err) => Err(err.into_static()),
        }
    }
}

/// Blanket [`ToBoundedStatic`] impl for converting `[T; const N: usize]` to `[T; const N: usize]: 'static`.
impl<T, const N: usize> ToBoundedStatic for [T; N]
where
    T: ToBoundedStatic + Copy,
{
    type Static = [T::Static; N];

    fn to_static(&self) -> Self::Static {
        // Note that we required that `T` is `Copy` here whereas the `IntoBoundedStatic` impl does does not.
        self.map(|item| item.to_static())
    }
}

/// Blanket [`IntoBoundedStatic`] impl for converting `[T; const N: usize]` into `[T; const N: usize]: 'static`.
impl<T, const N: usize> IntoBoundedStatic for [T; N]
where
    T: IntoBoundedStatic,
{
    type Static = [T::Static; N];

    fn into_static(self) -> Self::Static {
        self.map(IntoBoundedStatic::into_static)
    }
}

/// Blanket [`ToBoundedStatic`] impl for converting tuples `(T1, T2, ...)` to `(T1, T2, ..): 'static`.
macro_rules! tuple_to_static {
    () => ();
    ($($name:ident,)+) => {
        tuple_to_static! (
            @gen $($name,)+,
            concat!(
                "Blanket [`ToBoundedStatic`] impl for converting tuple `",
                stringify!(($($name,)+)), "` to `", stringify!(($($name,)+)), ": 'static `"
            )
        );
    };
    (@gen $($name:ident,)+, $doc:expr) => {
        #[doc = $doc]
        impl<$($name: ToBoundedStatic),+> ToBoundedStatic for ($($name,)+) {
            type Static = ($($name::Static,)+);
            #[allow(non_snake_case)]
            fn to_static(&self) -> Self::Static {
                let ($(ref $name,)+) = *self;
                ($($name.to_static(),)+)
            }
        }
        tuple_to_static! {@peel $($name,)+ }
    };
    (@peel $name:ident, $($other:ident,)*) => {tuple_to_static! { $($other,)* }};
}

/// Blanket [`IntoBoundedStatic`] impl for converting tuples `(T1, T2, ...)` into `(T1, T2, ..): 'static`.
macro_rules! tuple_into_static {
    () => ();
    ($($name:ident,)+) => {
        tuple_into_static! (
            @gen $($name,)+,
            concat!(
                "Blanket [`IntoBoundedStatic`] impl for converting tuple `",
                stringify!(($($name,)+)), "` into `", stringify!(($($name,)+)), ": 'static `"
            )
        );
    };
    (@gen $($name:ident,)+, $doc:expr) => {
        #[doc = $doc]
        impl<$($name: IntoBoundedStatic),+> IntoBoundedStatic for ($($name,)+) {
            type Static = ($($name::Static,)+);
            #[allow(non_snake_case)]
            fn into_static(self) -> Self::Static {
                let ($($name,)+) = self;
                ($($name.into_static(),)+)
            }
        }
        tuple_into_static! {@peel $($name,)+ }
    };
    (@peel $name:ident, $($other:ident,)*) => {tuple_into_static! { $($other,)* }};
}

tuple_to_static! { T11, T10, T9, T8, T7, T6, T5, T4, T3, T2, T1, T0, }
tuple_into_static! { T11, T10, T9, T8, T7, T6, T5, T4, T3, T2, T1, T0, }

#[cfg(feature = "alloc")]
/// Blanket [`ToBoundedStatic`] impl for converting `Cow<'a, T: ?Sized>` to `Cow<'static, T: ?Sized>`.
impl<T> ToBoundedStatic for Cow<'_, T>
where
    T: 'static + ToOwned + ?Sized,
{
    type Static = Cow<'static, T>;

    fn to_static(&self) -> Self::Static {
        Cow::Owned(self.clone().into_owned())
    }
}

#[cfg(feature = "alloc")]
/// Blanket [`IntoBoundedStatic`] impl for converting `Cow<'a, T: ?Sized>` into `Cow<'static, T: ?Sized>`.
impl<T> IntoBoundedStatic for Cow<'_, T>
where
    T: 'static + ToOwned + ?Sized,
{
    type Static = Cow<'static, T>;

    fn into_static(self) -> Self::Static {
        Cow::Owned(self.into_owned())
    }
}

#[cfg(feature = "alloc")]
/// [`ToBoundedStatic`] impl for `String`.
impl ToBoundedStatic for String {
    type Static = Self;

    fn to_static(&self) -> Self::Static {
        self.clone()
    }
}

#[cfg(feature = "alloc")]
/// No-op [`IntoBoundedStatic`] impl for `String`.
impl IntoBoundedStatic for String {
    type Static = Self;

    fn into_static(self) -> Self::Static {
        self
    }
}

#[cfg(feature = "alloc")]
/// Blanket [`ToBoundedStatic`] impl for converting `Vec<T>` to `Vec<T>: 'static`.
impl<T> ToBoundedStatic for Vec<T>
where
    T: ToBoundedStatic,
{
    type Static = Vec<T::Static>;

    fn to_static(&self) -> Self::Static {
        self.iter().map(ToBoundedStatic::to_static).collect()
    }
}

#[cfg(feature = "alloc")]
/// Blanket [`IntoBoundedStatic`] impl for converting `Vec<T>` into `Vec<T>: 'static`.
impl<T> IntoBoundedStatic for Vec<T>
where
    T: IntoBoundedStatic,
{
    type Static = Vec<T::Static>;

    fn into_static(self) -> Self::Static {
        self.into_iter()
            .map(IntoBoundedStatic::into_static)
            .collect()
    }
}

#[cfg(feature = "collections")]
/// Blanket [`ToBoundedStatic`] impl for converting `BinaryHeap<T>` into `BinaryHeap<T>: 'static`.
impl<T> ToBoundedStatic for BinaryHeap<T>
where
    T: ToBoundedStatic,
    T::Static: Ord,
{
    type Static = BinaryHeap<T::Static>;

    fn to_static(&self) -> Self::Static {
        self.iter().map(ToBoundedStatic::to_static).collect()
    }
}

#[cfg(feature = "collections")]
/// Blanket [`IntoBoundedStatic`] impl for converting `BinaryHeap<T>` into `BinaryHeap<T>: 'static`.
impl<T> IntoBoundedStatic for BinaryHeap<T>
where
    T: IntoBoundedStatic,
    T::Static: Ord,
{
    type Static = BinaryHeap<T::Static>;

    fn into_static(self) -> Self::Static {
        self.into_iter()
            .map(IntoBoundedStatic::into_static)
            .collect()
    }
}

#[cfg(feature = "collections")]
/// Blanket [`ToBoundedStatic`] impl for converting `BTreeMap<K, V>` into `BTreeMap<K, V>: 'static`.
impl<K, V> ToBoundedStatic for BTreeMap<K, V>
where
    K: ToBoundedStatic,
    K::Static: Ord,
    V: ToBoundedStatic,
{
    type Static = BTreeMap<K::Static, V::Static>;

    fn to_static(&self) -> Self::Static {
        self.iter()
            .map(|(k, v)| (k.to_static(), v.to_static()))
            .collect()
    }
}

#[cfg(feature = "collections")]
/// Blanket [`IntoBoundedStatic`] impl for converting `BTreeMap<K, V>` into `BTreeMap<K, V>: 'static`.
impl<K, V> IntoBoundedStatic for BTreeMap<K, V>
where
    K: IntoBoundedStatic,
    K::Static: Ord,
    V: IntoBoundedStatic,
{
    type Static = BTreeMap<K::Static, V::Static>;

    fn into_static(self) -> Self::Static {
        self.into_iter()
            .map(|(k, v)| (k.into_static(), v.into_static()))
            .collect()
    }
}

#[cfg(feature = "collections")]
/// Blanket [`ToBoundedStatic`] impl for converting `BTreeSet<T>` into `BTreeSet<T>: 'static`.
impl<T> ToBoundedStatic for BTreeSet<T>
where
    T: ToBoundedStatic,
    T::Static: Ord,
{
    type Static = BTreeSet<T::Static>;

    fn to_static(&self) -> Self::Static {
        self.iter().map(ToBoundedStatic::to_static).collect()
    }
}

#[cfg(feature = "collections")]
/// Blanket [`IntoBoundedStatic`] impl for converting `BTreeSet<T>` into `BTreeSet<T>: 'static`.
impl<T> IntoBoundedStatic for BTreeSet<T>
where
    T: IntoBoundedStatic,
    T::Static: Ord,
{
    type Static = BTreeSet<T::Static>;

    fn into_static(self) -> Self::Static {
        self.into_iter()
            .map(IntoBoundedStatic::into_static)
            .collect()
    }
}

#[cfg(feature = "collections")]
/// Blanket [`ToBoundedStatic`] impl for converting `LinkedList<T>` into `LinkedList<T>: 'static`.
impl<T> ToBoundedStatic for LinkedList<T>
where
    T: ToBoundedStatic,
{
    type Static = LinkedList<T::Static>;

    fn to_static(&self) -> Self::Static {
        self.iter().map(ToBoundedStatic::to_static).collect()
    }
}

#[cfg(feature = "collections")]
/// Blanket [`IntoBoundedStatic`] impl for converting `LinkedList<T>` into `LinkedList<T>: 'static`.
impl<T> IntoBoundedStatic for LinkedList<T>
where
    T: IntoBoundedStatic,
{
    type Static = LinkedList<T::Static>;

    fn into_static(self) -> Self::Static {
        self.into_iter()
            .map(IntoBoundedStatic::into_static)
            .collect()
    }
}

#[cfg(feature = "collections")]
/// Blanket [`ToBoundedStatic`] impl for converting `VecDeque<T>` into `VecDeque<T>: 'static`.
impl<T> ToBoundedStatic for VecDeque<T>
where
    T: ToBoundedStatic,
{
    type Static = VecDeque<T::Static>;

    fn to_static(&self) -> Self::Static {
        self.iter().map(ToBoundedStatic::to_static).collect()
    }
}

#[cfg(feature = "collections")]
/// Blanket [`IntoBoundedStatic`] impl for converting `VecDeque<T>` into `VecDeque<T>: 'static`.
impl<T> IntoBoundedStatic for VecDeque<T>
where
    T: IntoBoundedStatic,
{
    type Static = VecDeque<T::Static>;

    fn into_static(self) -> Self::Static {
        self.into_iter()
            .map(IntoBoundedStatic::into_static)
            .collect()
    }
}

#[cfg(feature = "alloc")]
/// Blanket [`ToBoundedStatic`] impl for converting `Box<T>` to `Box<T>: 'static`.
impl<T> ToBoundedStatic for Box<T>
where
    T: ToBoundedStatic,
{
    type Static = Box<T::Static>;

    fn to_static(&self) -> Self::Static {
        Box::new(self.as_ref().to_static())
    }
}

#[cfg(feature = "alloc")]
/// Blanket [`IntoBoundedStatic`] impl for converting `Box<T>` into `Box<T>: 'static`.
impl<T> IntoBoundedStatic for Box<T>
where
    T: IntoBoundedStatic,
{
    type Static = Box<T::Static>;

    fn into_static(self) -> Self::Static {
        Box::new((*self).into_static())
    }
}

#[cfg(feature = "std")]
/// Blanket [`ToBoundedStatic`] impl for converting `HashMap<K, V>` to `HashMap<K, V>: 'static`.
impl<K, V, S: std::hash::BuildHasher + Default + 'static> ToBoundedStatic
    for std::collections::HashMap<K, V, S>
where
    K: ToBoundedStatic,
    K::Static: Eq + std::hash::Hash,
    V: ToBoundedStatic,
{
    type Static = std::collections::HashMap<K::Static, V::Static, S>;

    fn to_static(&self) -> Self::Static {
        self.iter()
            .map(|(k, v)| (k.to_static(), v.to_static()))
            .collect()
    }
}

#[cfg(feature = "std")]
/// Blanket [`IntoBoundedStatic`] impl for for converting `HashMap<K, V>` into `HashMap<K, V>: 'static`.
impl<K, V, S: std::hash::BuildHasher + Default + 'static> IntoBoundedStatic
    for std::collections::HashMap<K, V, S>
where
    K: IntoBoundedStatic,
    K::Static: Eq + std::hash::Hash,
    V: IntoBoundedStatic,
{
    type Static = std::collections::HashMap<K::Static, V::Static, S>;

    fn into_static(self) -> Self::Static {
        self.into_iter()
            .map(|(k, v)| (k.into_static(), v.into_static()))
            .collect()
    }
}

#[cfg(feature = "std")]
/// Blanket [`ToBoundedStatic`] impl for converting `HashSet<T>` into `HashSet<T>: 'static`.
impl<T, S: std::hash::BuildHasher + Default + 'static> ToBoundedStatic
    for std::collections::HashSet<T, S>
where
    T: ToBoundedStatic,
    T::Static: Eq + std::hash::Hash,
{
    type Static = std::collections::HashSet<T::Static, S>;

    fn to_static(&self) -> Self::Static {
        self.iter().map(ToBoundedStatic::to_static).collect()
    }
}

#[cfg(feature = "std")]
/// Blanket [`IntoBoundedStatic`] impl for converting `HashSet<T>` into `HashSet<T>: 'static`.
impl<T, S: std::hash::BuildHasher + Default + 'static> IntoBoundedStatic
    for std::collections::HashSet<T, S>
where
    T: IntoBoundedStatic,
    T::Static: Eq + std::hash::Hash,
{
    type Static = std::collections::HashSet<T::Static, S>;

    fn into_static(self) -> Self::Static {
        self.into_iter()
            .map(IntoBoundedStatic::into_static)
            .collect()
    }
}

/// [`ToBoundedStatic`] impl for `smol_str::SmolStr`.
#[cfg(feature = "smol_str")]
impl ToBoundedStatic for smol_str::SmolStr {
    type Static = Self;

    fn to_static(&self) -> Self::Static {
        self.clone()
    }
}

/// No-op [`IntoBoundedStatic`] impl for `smol_str::SmolStr`.
#[cfg(feature = "smol_str")]
impl IntoBoundedStatic for smol_str::SmolStr {
    type Static = Self;

    fn into_static(self) -> Self::Static {
        self
    }
}

/// [`ToBoundedStatic`] impl for `smallvec::SmallVec`.
#[cfg(feature = "smallvec")]
impl<A> ToBoundedStatic for smallvec::SmallVec<A>
where
    A: smallvec::Array + 'static,
    A::Item: Clone,
{
    type Static = Self;

    fn to_static(&self) -> Self::Static {
        self.clone()
    }
}

/// No-op [`IntoBoundedStatic`] impl for `smallvec::SmallVec`.
#[cfg(feature = "smallvec")]
impl<A> IntoBoundedStatic for smallvec::SmallVec<A>
where
    A: smallvec::Array + 'static,
    A::Item: Clone,
{
    type Static = Self;

    fn into_static(self) -> Self::Static {
        self
    }
}

/// [`ToBoundedStatic`] impl for `smartstring::SmartString`.
#[cfg(feature = "smartstring")]
impl<Mode> ToBoundedStatic for smartstring::SmartString<Mode>
where
    Mode: smartstring::SmartStringMode + 'static,
{
    type Static = Self;

    fn to_static(&self) -> Self::Static {
        self.clone()
    }
}

/// No-op [`IntoBoundedStatic`] impl for `smartstring::SmartString`.
#[cfg(feature = "smartstring")]
impl<Mode> IntoBoundedStatic for smartstring::SmartString<Mode>
where
    Mode: smartstring::SmartStringMode + 'static,
{
    type Static = Self;

    fn into_static(self) -> Self::Static {
        self
    }
}

#[cfg(test)]
mod core_tests {
    use super::*;

    fn ensure_static<T: 'static>(t: T) {
        drop(t);
    }

    #[test]
    fn test_bool() {
        ensure_static(false.to_static());
    }

    #[test]
    fn test_char() {
        ensure_static('a'.to_static());
    }

    #[test]
    fn test_f32() {
        ensure_static(0.0f32.to_static());
    }

    #[test]
    fn test_f64() {
        ensure_static(0.0f64.to_static());
    }

    #[test]
    fn test_usize() {
        ensure_static(0usize.to_static());
    }

    #[test]
    fn test_u8() {
        ensure_static(0u8.to_static());
    }

    #[test]
    fn test_u16() {
        ensure_static(0u16.to_static());
    }

    #[test]
    fn test_u32() {
        ensure_static(0u32.to_static());
    }

    #[test]
    fn test_u64() {
        ensure_static(0u64.to_static());
    }

    #[test]
    fn test_u128() {
        ensure_static(0u128.to_static());
    }

    #[test]
    fn test_isize() {
        ensure_static(0isize.to_static());
    }

    #[test]
    fn test_i8() {
        ensure_static(0i8.to_static());
    }

    #[test]
    fn test_i16() {
        ensure_static(0i16.to_static());
    }

    #[test]
    fn test_i32() {
        ensure_static(0i32.to_static());
    }

    #[test]
    fn test_i64() {
        ensure_static(0i64.to_static());
    }

    #[test]
    fn test_i128() {
        ensure_static(0i128.to_static());
    }

    #[test]
    fn test_non_zero_usize() {
        ensure_static(NonZeroUsize::new(1).unwrap().to_static());
    }

    #[test]
    fn test_non_zero_u8() {
        ensure_static(NonZeroU8::new(1).unwrap().to_static());
    }

    #[test]
    fn test_non_zero_u16() {
        ensure_static(NonZeroU16::new(1).unwrap().to_static());
    }

    #[test]
    fn test_non_zero_u32() {
        ensure_static(NonZeroU32::new(1).unwrap().to_static());
    }

    #[test]
    fn test_non_zero_u64() {
        ensure_static(NonZeroU64::new(1).unwrap().to_static());
    }

    #[test]
    fn test_non_zero_u128() {
        ensure_static(NonZeroU128::new(1).unwrap().to_static());
    }

    #[test]
    fn test_non_zero_isize() {
        ensure_static(NonZeroIsize::new(1).unwrap().to_static());
    }

    #[test]
    fn test_non_zero_i8() {
        ensure_static(NonZeroI8::new(1).unwrap().to_static());
    }

    #[test]
    fn test_non_zero_i16() {
        ensure_static(NonZeroI16::new(1).unwrap().to_static());
    }

    #[test]
    fn test_non_zero_i32() {
        ensure_static(NonZeroI32::new(1).unwrap().to_static());
    }

    #[test]
    fn test_non_zero_i64() {
        ensure_static(NonZeroI64::new(1).unwrap().to_static());
    }

    #[test]
    fn test_non_zero_i128() {
        ensure_static(NonZeroI128::new(1).unwrap().to_static());
    }

    #[test]
    fn test_unit() {
        #[allow(clippy::unit_arg)]
        ensure_static(().to_static());
    }

    #[test]
    fn test_str() {
        let s = "";
        let to_static = s.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_option_none() {
        let value: Option<u32> = None;
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_option_some() {
        let value: Option<u32> = Some(32);
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_result() {
        #[derive(Clone)]
        struct MyError;
        #[allow(clippy::unnecessary_wraps)]
        fn foo_ok() -> Result<(), MyError> {
            Ok(())
        }
        #[allow(clippy::unnecessary_wraps)]
        fn foo_err() -> Result<(), MyError> {
            Err(MyError)
        }
        impl ToBoundedStatic for MyError {
            type Static = Self;

            fn to_static(&self) -> Self::Static {
                self.clone()
            }
        }
        let ok_result = foo_ok();
        ensure_static(ok_result.to_static());
        assert!(ok_result.is_ok());
        let err_result = foo_err();
        ensure_static(err_result.to_static());
        assert!(err_result.is_err());
    }

    #[test]
    fn test_array() {
        let arr = ["test"];
        ensure_static(arr.to_static());
    }

    #[test]
    fn test_tuple2() {
        let tuple = ("test", 32);
        ensure_static(tuple.to_static());
    }

    #[test]
    fn test_tuple11() {
        let tuple = (
            (),
            '1',
            "2",
            3_i32,
            4_usize,
            5_isize,
            6.0_f64,
            ["7"],
            Some(8),
            9,
            (10,),
            false,
        );
        ensure_static(tuple.to_static());
    }
}

#[cfg(feature = "alloc")]
#[cfg(test)]
mod alloc_tests {
    use super::*;

    fn ensure_static<T: 'static>(t: T) {
        drop(t);
    }

    #[test]
    fn test_string() {
        let s = String::new();
        let to_static = s.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_cow_borrowed_str() {
        let s = String::new();
        let to_static = Cow::from(&s).to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_cow_owned_string() {
        let s = String::new();
        let to_static = Cow::from(s).to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_cow_to_static() {
        let s = String::new();
        let s_cow: Cow<'_, str> = Cow::Borrowed(&s);
        let s1_cow_owned: Cow<'_, str> = s_cow.to_static();
        let s2_cow_owned: Cow<'_, str> = Cow::Owned(s_cow.into_owned());
        assert_eq!(s1_cow_owned, s2_cow_owned);
    }

    #[test]
    fn test_cow_into_static() {
        let s = String::new();
        let s_cow: Cow<'_, str> = Cow::Borrowed(&s);
        let s1_cow_owned: Cow<'_, str> = s_cow.clone().into_static();
        let s2_cow_owned: Cow<'_, str> = Cow::Owned(s_cow.into_owned());
        assert_eq!(s1_cow_owned, s2_cow_owned);
    }

    #[test]
    fn test_option_none() {
        let value: Option<Cow<'_, str>> = None;
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_option_some() {
        let s = String::new();
        let value = Some(Cow::from(&s));
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_array() {
        let arr = ["test"];
        ensure_static(arr.to_static());
    }

    #[test]
    fn test_array_into() {
        let s = String::new();
        let arr = [Cow::from(&s)];
        ensure_static(arr.into_static());
    }

    #[test]
    fn test_vec1() {
        let s = String::new();
        let value = alloc::vec![Cow::from(&s)];
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_vec2() {
        let s = String::new();
        let value = alloc::vec![Cow::from(&s), Cow::from(s.as_str())];
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_box() {
        let s = String::new();
        let value = Box::new(s);
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_box_cow() {
        let s = String::new();
        let value = Box::new(Cow::from(&s));
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_box_vec_cow() {
        let s = String::new();
        let value = Box::new(alloc::vec![Cow::from(&s)]);
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_vec_box_cow() {
        let s = String::new();
        let value = alloc::vec![Box::new(Cow::from(&s))];
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_cow_box() {
        let s = String::new();
        let boxed = Box::new(s);
        let value = Cow::Borrowed(&boxed);
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_cow_struct() {
        #[derive(Copy, Clone)]
        struct Foo {}
        impl ToBoundedStatic for Foo {
            type Static = Self;

            fn to_static(&self) -> Self::Static {
                *self
            }
        }
        let foo = Foo {};
        let value = Cow::Borrowed(&foo);
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_cow_struct_of_cow() {
        #[derive(Clone)]
        struct Foo<'a> {
            foo: Cow<'a, str>,
        }
        impl ToBoundedStatic for Foo<'_> {
            type Static = Foo<'static>;

            fn to_static(&self) -> Self::Static {
                Foo {
                    foo: self.foo.to_static(),
                }
            }
        }
        let s = String::new();
        let foo = Foo { foo: Cow::from(&s) };
        let value = Cow::Borrowed(&foo);
        // TODO need to `into_owned()` here
        let to_static = value.into_owned().to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_cow_cow() {
        let s = String::new();
        let value1: Cow<'_, str> = Cow::Borrowed(&s);
        let value2: Cow<'_, Cow<'_, str>> = Cow::Borrowed(&value1);
        // TODO need to `into_owned()` here
        let to_static = value2.into_owned().to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_struct_cow_borrowed_str() {
        struct Foo<'a> {
            foo: Cow<'a, str>,
        }
        impl ToBoundedStatic for Foo<'_> {
            type Static = Foo<'static>;

            fn to_static(&self) -> Self::Static {
                Foo {
                    foo: self.foo.to_static(),
                }
            }
        }
        let s = String::new();
        let foo = Foo { foo: Cow::from(&s) };
        let to_static = foo.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_struct_cow_owned_string() {
        struct Foo<'a> {
            foo: Cow<'a, str>,
        }
        impl ToBoundedStatic for Foo<'_> {
            type Static = Foo<'static>;

            fn to_static(&self) -> Self::Static {
                Foo {
                    foo: self.foo.to_static(),
                }
            }
        }
        let s = String::new();
        let foo = Foo { foo: Cow::from(s) };
        let to_static = foo.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_struct_multi() {
        #[derive(Clone)]
        struct Foo<'a> {
            bar: Cow<'a, str>,
            baz: Vec<Cow<'a, str>>,
        }
        impl ToBoundedStatic for Foo<'_> {
            type Static = Foo<'static>;

            fn to_static(&self) -> Self::Static {
                Foo {
                    bar: self.bar.to_static(),
                    baz: self.baz.to_static(),
                }
            }
        }
        let s = String::new();
        let foo = Foo {
            bar: Cow::from(&s),
            baz: alloc::vec![Cow::from(&s)],
        };
        let to_static = foo.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_struct_mixed() {
        struct Foo<'a> {
            prim: u64,
            borrowed_str: &'static str,
            owned_str: String,
            cow_str: Cow<'a, str>,
        }
        impl ToBoundedStatic for Foo<'_> {
            type Static = Foo<'static>;

            fn to_static(&self) -> Self::Static {
                Foo {
                    prim: self.prim.to_static(),
                    borrowed_str: self.borrowed_str.to_static(),
                    owned_str: self.owned_str.to_static(),
                    cow_str: self.cow_str.to_static(),
                }
            }
        }
        let s = String::new();
        let foo = Foo {
            prim: 0,
            borrowed_str: "",
            owned_str: s.clone(),
            cow_str: Cow::from(&s),
        };
        let to_static = foo.to_static();
        ensure_static(to_static);
    }
}

#[cfg(feature = "collections")]
#[cfg(test)]
mod collections_tests {
    use super::*;

    fn ensure_static<T: 'static>(t: T) {
        drop(t);
    }

    #[test]
    fn test_binary_heap() {
        let s = String::new();
        let value = BinaryHeap::from([Cow::from(&s)]);
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_btree_map() {
        let k = String::from("key");
        let v = String::from("value");
        let value = BTreeMap::from([(Cow::from(&k), Cow::from(&v))]);
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_btree_set() {
        let s = String::new();
        let value = BTreeSet::from([Cow::from(&s)]);
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_linked_list() {
        let s = String::new();
        let value = LinkedList::from([Cow::from(&s)]);
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_vec_deque() {
        let s = String::new();
        let value = VecDeque::from([Cow::from(&s)]);
        let to_static = value.to_static();
        ensure_static(to_static);
    }
}

#[cfg(feature = "std")]
#[cfg(test)]
mod std_tests {
    use core::any::Any;

    use super::*;

    fn ensure_static<T: 'static>(t: T) {
        drop(t);
    }

    #[test]
    fn test_hashmap1() {
        let k = String::from("key");
        let v = String::from("value");
        let value = std::collections::HashMap::from([(Cow::from(&k), Cow::from(&v))]);
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_hashmap2() {
        let k = "key";
        let v = String::from("value");
        let value = std::collections::HashMap::from([(k, Cow::from(&v))]);
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_hashmap3() {
        let k = String::from("key");
        let v = 0i16;
        let value = std::collections::HashMap::from([(Cow::from(&k), v)]);
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_custom_random_state() {
        struct RandomState;

        impl std::hash::BuildHasher for RandomState {
            type Hasher = std::collections::hash_map::DefaultHasher;

            fn build_hasher(&self) -> Self::Hasher {
                std::collections::hash_map::DefaultHasher::default()
            }
        }

        impl Default for RandomState {
            fn default() -> Self {
                Self
            }
        }

        let k = "key";
        let v = 0i16;
        let value = std::collections::HashMap::<_, _, RandomState>::from_iter([(k, v)]);
        let to_static = value.to_static();
        assert_eq!(value.type_id(), to_static.type_id());
        ensure_static(to_static);
        let value = std::collections::HashSet::<_, RandomState>::from_iter([k]);
        let to_static = value.to_static();
        assert_eq!(value.type_id(), to_static.type_id());
        ensure_static(to_static);
    }

    #[test]
    fn test_hashset() {
        let value = String::from("data");
        let value = std::collections::HashSet::from([(Cow::from(&value))]);
        let to_static = value.to_static();
        ensure_static(to_static);
    }
}

#[cfg(feature = "smol_str")]
#[cfg(test)]
mod smol_str_tests {
    use super::*;

    fn ensure_static<T: 'static>(t: T) {
        drop(t);
    }

    #[test]
    fn test_smol_str() {
        ensure_static(smol_str::SmolStr::new("smol").to_static());
        ensure_static(smol_str::SmolStr::new("smol").into_static());
    }
}

#[cfg(feature = "smallvec")]
#[cfg(test)]
mod smallvec_tests {
    use super::*;

    fn ensure_static<T: 'static>(t: T) {
        drop(t);
    }

    #[test]
    fn test_smallvec1() {
        let vec: smallvec::SmallVec<[usize; 0]> = smallvec::SmallVec::new();
        ensure_static(vec.to_static());
        ensure_static(vec.into_static());
    }

    #[test]
    fn test_smallvec2() {
        let buf = [1, 2, 3, 4, 5];
        let small_vec: smallvec::SmallVec<_> = smallvec::SmallVec::from_buf(buf);
        ensure_static(small_vec.to_static());
        ensure_static(small_vec.into_static());
    }
}

#[cfg(feature = "smartstring")]
#[cfg(test)]
mod smartstring_tests {
    use super::*;
    use smartstring::alias::String;

    fn ensure_static<T: 'static>(t: T) {
        drop(t);
    }

    #[test]
    fn test_smartstring() {
        let string = String::from("test");
        ensure_static(string.to_static());
        ensure_static(string.into_static());
    }
}
