//! Resolved types of binary pattern segments (`<<len:8, payload: bin(len), rest: bin>>`).

use rustc_hir::BinarySegmentTy;
use rustc_macros::{StableHash, TyDecodable, TyEncodable};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, TyEncodable, TyDecodable, StableHash)]
pub enum BinaryEndianness {
    Big,
    Little,
    Native,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, TyEncodable, TyDecodable, StableHash)]
pub enum BinaryEncoding {
    Utf8,
    Utf16,
    Utf32,
}

/// The size of a binary pattern segment, in units of the segment's `unit`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, TyEncodable, TyDecodable, StableHash)]
pub enum BinarySegmentSize<E> {
    /// A size known at compile time.
    Fixed(u64),
    /// A size given by an expression evaluated at runtime.
    Dynamic(E),
    /// The size of the segment's own value, known at runtime.
    OfValue,
    /// No size: the segment consumes the rest of the matched bitstring.
    Unsized,
    /// The size of an encoded character or string literal, which comes from its encoding.
    Encoded(BinaryEncoding),
}

/// The type of a binary pattern segment with all defaults applied.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, TyEncodable, TyDecodable, StableHash)]
pub struct BinarySegmentLayout<E> {
    pub ty: BinarySegmentTy,
    pub size: BinarySegmentSize<E>,
    pub unit: u16,
    pub signed: bool,
    pub endianness: BinaryEndianness,
}

impl<E> BinarySegmentLayout<E> {
    pub fn with_size_expr<F>(self, f: impl FnOnce(E) -> F) -> BinarySegmentLayout<F> {
        let Self { ty, size, unit, signed, endianness } = self;
        let size = match size {
            BinarySegmentSize::Fixed(size) => BinarySegmentSize::Fixed(size),
            BinarySegmentSize::Dynamic(expr) => BinarySegmentSize::Dynamic(f(expr)),
            BinarySegmentSize::OfValue => BinarySegmentSize::OfValue,
            BinarySegmentSize::Unsized => BinarySegmentSize::Unsized,
            BinarySegmentSize::Encoded(encoding) => BinarySegmentSize::Encoded(encoding),
        };
        BinarySegmentLayout { ty, size, unit, signed, endianness }
    }

    /// The set of possible sizes of this segment in bits, or `None` for an unsized tail.
    pub fn size_class(&self) -> Option<BinarySizeClass> {
        let unit = u64::from(self.unit);
        match self.size {
            BinarySegmentSize::Fixed(size) => Some(
                size.checked_mul(unit)
                    .map_or(BinarySizeClass::multiple_of(unit), BinarySizeClass::exact),
            ),
            BinarySegmentSize::Dynamic(_) | BinarySegmentSize::OfValue => {
                Some(BinarySizeClass::multiple_of(unit))
            }
            BinarySegmentSize::Encoded(BinaryEncoding::Utf8) => {
                Some(BinarySizeClass::multiple_of(8))
            }
            BinarySegmentSize::Encoded(BinaryEncoding::Utf16) => {
                Some(BinarySizeClass::multiple_of(16))
            }
            BinarySegmentSize::Encoded(BinaryEncoding::Utf32) => {
                Some(BinarySizeClass::multiple_of(32))
            }
            BinarySegmentSize::Unsized => None,
        }
    }

    /// The encoding of a character or string literal segment.
    pub fn encoding(&self) -> Option<BinaryEncoding> {
        match self.size {
            BinarySegmentSize::Encoded(encoding) => Some(encoding),
            BinarySegmentSize::Fixed(_)
            | BinarySegmentSize::Dynamic(_)
            | BinarySegmentSize::OfValue
            | BinarySegmentSize::Unsized => None,
        }
    }
}

/// A set of bit sizes `{ residue + k * modulus | k >= 0 }`.
///
/// Used to prove that a binary pattern expression builds a whole number of bytes, and so is a
/// `bin`. A modulus of 0 means the size is exactly `residue`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct BinarySizeClass {
    residue: u64,
    modulus: u64,
}

impl BinarySizeClass {
    pub const ZERO: Self = Self { residue: 0, modulus: 0 };

    /// Any size at all.
    pub const ANY: Self = Self { residue: 0, modulus: 1 };

    pub fn exact(size: u64) -> Self {
        Self { residue: size, modulus: 0 }
    }

    pub fn multiple_of(modulus: u64) -> Self {
        Self { residue: 0, modulus }
    }

    /// The possible sizes of two consecutive segments.
    pub fn add(self, other: Self) -> Self {
        let modulus = gcd(self.modulus, other.modulus);
        let residue = u128::from(self.residue) + u128::from(other.residue);
        if modulus == 0 {
            // Two exact sizes too large to add up are still some size.
            u64::try_from(residue).map_or(Self::ANY, Self::exact)
        } else {
            let residue = (residue % u128::from(modulus)) as u64;
            Self { residue, modulus }
        }
    }

    /// Is every size in this set divisible by `n`?
    pub fn is_multiple_of(self, n: u64) -> bool {
        self.modulus % n == 0 && self.residue % n == 0
    }
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}
