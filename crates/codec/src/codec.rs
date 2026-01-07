///
///
/// the trait Codec is not dyn compatible：
/// 一个 trait 要能用作 dyn Trait，
/// 它的所有方法都必须在 vtable 中是“确定签名”的

pub trait Codec: Send + Sync {
    fn encode<T: ?Sized>(&self, v: &T) -> Result<Vec<u8>, String>;
    fn decode<T>(&self, bytes: &Vec<u8>) -> Result<T, String>;
}
