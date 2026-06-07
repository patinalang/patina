rustc_index::newtype_index! {
    /// The *source-order* index of a field in a variant.
    ///
    /// This is how most code after type checking refers to fields, rather than
    /// using names (as names have hygiene complications and more complex lookup).
    ///
    /// Particularly for `repr(Rust)` types, this may not be the same as *layout* order.
    /// (It is for `repr(C)` `struct`s, however.)
    ///
    /// For example, in the following types,
    /// ```rust
    /// # enum Never {}
    /// # #[repr(u16)]
    /// enum Demo1 {
    ///    Variant0 { a: Never, b: i32 } = 100,
    ///    Variant1 { c: u8, d: u64 } = 10,
    /// }
    /// struct Demo2 { e: u8, f: u16, g: u8 }
    /// ```
    /// `b` is `FieldIdx(1)` in `VariantIdx(0)`,
    /// `d` is `FieldIdx(1)` in `VariantIdx(1)`, and
    /// `f` is `FieldIdx(1)` in `VariantIdx(0)`.
    #[stable_hash]
    #[encodable]
    #[orderable]
    #[gate_rustc_only]
    pub struct FieldIdx {}
}

impl FieldIdx {
    /// The second field, at index 1.
    ///
    /// For use alongside [`FieldIdx::ZERO`], particularly with scalar pairs.
    pub const ONE: FieldIdx = FieldIdx::from_u32(1);
}

rustc_index::newtype_index! {
    /// The *source-order* index of a variant in a type.
    ///
    /// For enums, these are always `0..variant_count`, regardless of any
    /// custom discriminants that may have been defined, and including any
    /// variants that may end up uninhabited due to field types.  (Some of the
    /// variants may not be present in a monomorphized ABI [`Variants`], but
    /// those skipped variants are always counted when determining the *index*.)
    ///
    /// `struct`s, `tuples`, and `unions`s are considered to have a single variant
    /// with variant index zero, aka [`FIRST_VARIANT`].
    #[stable_hash]
    #[encodable]
    #[orderable]
    #[gate_rustc_only]
    pub struct VariantIdx {
        /// Equivalent to `VariantIdx(0)`.
        const FIRST_VARIANT = 0;
    }
}
