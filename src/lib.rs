// Use `README.md` as documentation home page, to reduce duplication
#![doc = include_str!("../README.md")]

/// Types whose values can be duplicated simply by copying bits without worrying about provenance.
///
/// The trait flags plain data types, without the move semantics that [`Copy`] entails.
/// Values of these types can also be turned into trait objects,
/// because [`Copy`] is not a supertrait of [`Unscrupulous`].
///
/// Implementors include primitives like [`i32`] or [`bool`], but excludes references and pointers.
/// This is quite restrictive, even more than [`Copy`] (though less than [`bytemuck::Pod`]).
///
/// # Safety
///
/// All nested fields of the type must also be [`Unscrupulous`].
/// Use the derive macro provided by the `derive` feature to check this invariant at compile time.
///
/// [`bytemuck::Pod`]: https://docs.rs/bytemuck/latest/bytemuck/trait.Pod.html
pub unsafe trait Unscrupulous {}
