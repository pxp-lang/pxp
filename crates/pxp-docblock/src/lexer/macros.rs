#[macro_export]
macro_rules! inline_whitespace {
    () => { b' ' | b'\t' };
}

#[macro_export]
macro_rules! whitespace {
    () => { b' ' | b'\t' | b'\n' | b'\r' };
}