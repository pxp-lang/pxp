use pxp_bytestring::ByteString;

#[derive(Debug, Clone)]
pub(crate) struct Constant {
    pub name: ByteString,
    pub short: ByteString,
    pub namespace: Option<ByteString>,
}
