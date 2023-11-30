#[macro_export]
macro_rules! whitespace {
    () => { b' ' | b'\t' |  b'\n' | b'\r' };
}

#[macro_export]
macro_rules! ident_start {
    () => {
        b'a'..=b'z' | b'A'..=b'Z' | b'_' | b'\x80'..=b'\xff'
    };
}

#[macro_export]
macro_rules! ident {
    () => {
        b'0'..=b'9' | b'a'..=b'z' | b'A'..=b'Z' | b'_' | b'\x80'..=b'\xff'
    };
}

#[macro_export]
macro_rules! tag {
    () => {
        b'a'..=b'z' | b'0'..=b'9' | b'-' | b':' | b'\x80'..=b'\xff' | b'\\'
    };
}