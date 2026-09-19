pub(crate) fn trim_trailing_zeroes(bytes: &[u8]) -> &[u8] {
    if bytes.is_empty() {
        return bytes;
    }
    let end = bytes.iter().rposition(|&b| b != 0).map(|i| i + 1).unwrap_or(1);
    &bytes[..end]
}
