use std::{fmt::{Display, Debug}, str::FromStr, ops::{Deref, DerefMut}};

/// A custom non UTF-8 string type that wraps a bunch of bytes.
/// 
/// PHP source code does not need to be UTF-8 compliant.
/// This structure provides a way to store, manipulate and display a sequence of bytes.
#[derive(PartialEq, Eq, PartialOrd, Clone, Hash, Default)]
pub struct ByteString {
    bytes: Vec<u8>,    
}

impl ByteString {
    pub fn new<B: Into<Vec<u8>>>(bytes: B) -> Self {
        Self { bytes: bytes.into() }
    }
}

impl Display for ByteString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for &b in &self.bytes {
            match b {
                0 => write!(f, "\\0")?,
                b'\n' | b'\r' | b'\t' => write!(f, "{}", b as char)?,
                0x01..=0x19 | 0x7f..=0xff => write!(f, "\\x{:02x}", b)?,
                _ => write!(f, "{}", b as char)?,
            }
        }

        Ok(())
    }
}

impl Debug for ByteString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"")?;
        
        for &b in &self.bytes {
            match b {
                0 => write!(f, "\\0")?,
                b'\n' | b'\r' | b'\t' => write!(f, "{}", b.escape_ascii())?,
                0x01..=0x19 | 0x7f..=0xff => write!(f, "\\x{:02x}", b)?,
                _ => write!(f, "{}", b as char)?,
            }
        }
        
        write!(f, "\"")?;

        Ok(())
    }
}

impl FromStr for ByteString {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ByteString::new(s.as_bytes()))
    }
}

impl<const N: usize> PartialEq<&[u8; N]> for ByteString {
    fn eq(&self, other: &&[u8; N]) -> bool {
        &self.bytes == other
    }
}

impl PartialEq<&[u8]> for ByteString {
    fn eq(&self, other: &&[u8]) -> bool {
        &self.bytes == other
    }
}

impl<const N: usize> PartialEq<&[u8; N]> for &ByteString {
    fn eq(&self, other: &&[u8; N]) -> bool {
        &self.bytes == other
    }
}

impl PartialEq<&[u8]> for &ByteString {
    fn eq(&self, other: &&[u8]) -> bool {
        &self.bytes == other
    }
}

impl PartialEq<String> for ByteString {
    fn eq(&self, other: &String) -> bool {
        &self.bytes == other.as_bytes()
    }
}

impl PartialEq<&str> for ByteString {
    fn eq(&self, other: &&str) -> bool {
        &self.bytes == other.as_bytes()
    }
}

impl From<u8> for ByteString {
    fn from(byte: u8) -> Self {
        ByteString::new(vec![byte])
    }
}

impl From<Vec<u8>> for ByteString {
    fn from(bytes: Vec<u8>) -> Self {
        ByteString::new(bytes)
    }
}

impl From<&[u8]> for ByteString {
    fn from(bytes: &[u8]) -> Self {
        ByteString::new(bytes.to_vec())
    }
}

impl<const N: usize> From<&[u8; N]> for ByteString {
    fn from(bytes: &[u8; N]) -> Self {
        ByteString::new(bytes.to_vec())
    }
}

impl From<&str> for ByteString {
    fn from(bytes: &str) -> Self {
        ByteString::new(bytes.as_bytes())
    }
}

impl From<String> for ByteString {
    fn from(bytes: String) -> Self {
        ByteString::new(bytes.into_bytes())
    }
}

impl Deref for ByteString {
    type Target = Vec<u8>;

    fn deref(&self) -> &Vec<u8> {
        &self.bytes
    }
}

impl DerefMut for ByteString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_be_constructed() {
        assert!(matches!(ByteString::default(), ByteString { .. }));
        assert!(matches!(ByteString::new(Vec::new()), ByteString { .. }));
    }

    #[test]
    fn it_can_be_constructed_from_a_slice() {
        let bytes = b"Hello, world!";

        assert_eq!(ByteString::from(bytes), ByteString::new(bytes.to_vec()));
    }

    #[test]
    fn it_can_be_constructed_from_a_string_slice() {
        let bytes = "Hello, world!";

        assert_eq!(ByteString::from(bytes), ByteString::new(bytes.as_bytes().to_vec()));
    }

    #[test]
    fn it_can_be_constructed_from_a_string() {
        let bytes = "Hello, world!".to_string();

        assert_eq!(ByteString::from(bytes.clone()), ByteString::new(bytes.into_bytes()));
    }

    #[test]
    fn it_can_be_constructed_from_a_byte() {
        let byte = b'H';

        assert_eq!(ByteString::from(byte), ByteString::new(vec![byte]));
    }

    #[test]
    fn it_can_be_constructed_from_a_byte_array() {
        let bytes = [b'H', b'e', b'l', b'l', b'o'];

        assert_eq!(ByteString::from(&bytes), ByteString::new(bytes.to_vec()));
    }

    #[test]
    fn it_can_be_constructed_from_a_byte_array_slice() {
        let bytes = [b'H', b'e', b'l', b'l', b'o'];

        assert_eq!(ByteString::from(&bytes[..]), ByteString::new(bytes.to_vec()));
    }

    #[test]
    fn it_can_be_displayed() {
        let bytes = b"Hello, world!";

        assert_eq!(format!("{}", ByteString::from(bytes)), "Hello, world!");
    }

    #[test]
    fn it_can_be_debugged() {
        let bytes = b"Hello, world!";

        assert_eq!(format!("{:?}", ByteString::from(bytes)), "\"Hello, world!\"");
    }

    #[test]
    fn it_can_be_compared_to_a_byte_array() {
        let bytes = b"Hello, world!";

        assert_eq!(ByteString::from(bytes), bytes);
    }

    #[test]
    fn it_can_be_compared_to_a_byte_array_slice() {
        let bytes = b"Hello, world!";

        assert_eq!(ByteString::from(bytes), &bytes[..]);
    }

    #[test]
    fn it_can_be_compared_to_a_byte_array_slice_ref() {
        let bytes = b"Hello, world!";

        assert_eq!(&ByteString::from(bytes), &bytes[..]);
    }

    #[test]
    fn it_can_be_compared_to_a_byte_array_ref() {
        let bytes = b"Hello, world!";

        assert_eq!(&ByteString::from(bytes), bytes);
    }

    #[test]
    fn it_can_be_compared_to_a_string_slice() {
        let bytes = b"Hello, world!";

        assert_eq!(ByteString::from(bytes), "Hello, world!");
    }

    #[test]
    fn it_can_be_compared_to_a_string() {
        let bytes = b"Hello, world!";

        assert_eq!(ByteString::from(bytes), "Hello, world!".to_string());
    }
}