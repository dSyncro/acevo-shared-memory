//! Internal utilities for interpreting raw shared-memory data.

/// Interprets a null-terminated `i8` buffer from a C struct as a Rust `&str`.
///
/// The buffer is reinterpreted as `u8`, truncated at the first null byte, and
/// decoded as UTF-8. Returns `""` if the bytes are not valid UTF-8.
pub fn parse_c_str(buf: &[i8]) -> &str {
    let bytes = unsafe { std::slice::from_raw_parts(buf.as_ptr() as *const u8, buf.len()) };
    let end = bytes.iter().position(|&b| b == 0).unwrap_or(buf.len());
    std::str::from_utf8(&bytes[..end]).unwrap_or("")
}
