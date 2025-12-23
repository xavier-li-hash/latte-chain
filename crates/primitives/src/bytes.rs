#[derive(Clone,Debug,PartialEq,Eq,Hash)]
pub struct Bytes(pub Vec<u8>);

impl Bytes {
    pub fn new(data:Vec<u8>) -> Self {
        Bytes(data)
    }
    pub fn empty() -> Self {
        Bytes(Vec::new())
    }
    pub fn length(&self) -> usize {
        self.0.len()
    }
    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}

impl From<Vec<u8>> for Bytes {
    fn from(v: Vec<u8>) -> Self {
        Bytes(v)
    }
}