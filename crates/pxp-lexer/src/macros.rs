// Reusable pattern for the first byte of an identifier.
#[macro_export]
macro_rules! ident_start {
    () => {
        b'a'..=b'z' | b'A'..=b'Z' | b'_' | b'\x80'..=b'\xff'
    };
}

// Reusable pattern for identifier after the first byte.
#[macro_export]
macro_rules! ident {
    () => {
        b'0'..=b'9' | b'a'..=b'z' | b'A'..=b'Z' | b'_' | b'\x80'..=b'\xff'
    };
}
