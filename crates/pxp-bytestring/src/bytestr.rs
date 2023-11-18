use std::ops::Deref;

use crate::ByteString;

#[derive(PartialOrd, PartialEq, Eq, Clone, Hash)]
pub struct ByteStr<'a> {
    pub bytes: &'a [u8],
}

impl<'a> ByteStr<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        ByteStr { bytes }
    }

    pub fn to_bytestring(&self) -> ByteString {
        ByteString::from(self)
    }
}

impl<'a> Default for ByteStr<'a> {
    fn default() -> Self {
        ByteStr::new(&[])
    }
}

impl<'a> std::fmt::Display for ByteStr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for &b in self.bytes {
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

impl<'a> From<&'a str> for ByteStr<'a> {
    fn from(value: &'a str) -> Self {
        ByteStr::new(value.as_bytes())
    }
}

impl<'a> std::fmt::Debug for ByteStr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"")?;
        for &b in self.bytes {
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

impl<'a, const N: usize> PartialEq<&[u8; N]> for ByteStr<'a> {
    fn eq(&self, other: &&[u8; N]) -> bool {
        &self.bytes == other
    }
}

impl<'a, const N: usize> PartialEq<&[u8; N]> for &ByteStr<'a> {
    fn eq(&self, other: &&[u8; N]) -> bool {
        &self.bytes == other
    }
}

impl<'a> From<&'a [u8]> for ByteStr<'a> {
    fn from(value: &'a [u8]) -> Self {
        ByteStr::new(value)
    }
}

impl<'a, const N: usize> From<&'a [u8; N]> for ByteStr<'a> {
    fn from(bytes: &'a [u8; N]) -> Self {
        ByteStr::new(bytes)
    }
}

impl<'a> Deref for ByteStr<'_> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.bytes
    }
}
