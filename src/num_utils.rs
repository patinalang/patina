use num_traits::{PrimInt, ToBytes, WrappingNeg};

pub(crate) fn to_sign_mag<T: PrimInt + ToBytes + WrappingNeg>(v: T) -> (u8, <T as ToBytes>::Bytes) {
    let sign = if v < T::zero() { 1u8 } else { 0u8 };
    let mag = if sign == 1 { v.wrapping_neg() } else { v };
    (sign, mag.to_le_bytes())
}

pub(crate) fn trim_trailing_zeroes(bytes: &[u8]) -> &[u8] {
    let end = bytes.iter().rposition(|&b| b != 0).map(|i| i + 1).unwrap_or(0);
    &bytes[..end]
}
