
use ed25519_dalek::{
    SigningKey, VerifyingKey, Signature, Signer, Verifier
};
use rand::rngs::OsRng;


pub struct Keypair {
    pub signing: SigningKey, // 零化处理 (Zeroing): ed25519-dalek 的 SigningKey 在内存中被销毁时会自动擦除数据，防止内存嗅探攻击泄露私钥。
    pub verifying: VerifyingKey,
}

impl Keypair {
    // 生成密钥对
    pub fn generate() -> Self {
        // 生成私钥
        let signing = SigningKey::generate(&mut OsRng);
        // 在Ed25519中，公钥是从私钥中通过椭圆曲线运算派生出来的。这意味着你只需要保存私钥，就能随时找回公钥
        let verifying = signing.verifying_key();
        Self { signing, verifying }
    }

    // 使用私钥签名
    // 结合私钥和消息内容，生成一段只有持有该私钥的人才能产生的证明。如果消息内容哪怕改变了一个字节，生成的签名也会完全不同。
    pub fn sign(&self, msg: &[u8]) -> Signature {
        self.signing.sign(msg)
    }
}


/// 它接收公钥、原始消息和签名。如果验证通过（返回 true），则可以百分之百确定：
/// 消息在签名后未被篡改。
/// 签名确实是由该公钥对应的私钥持有人签署的。
///
/// # Arguments
///
/// # Returns
/// 返回校验结果
pub fn verify(
    pubkey: &VerifyingKey,
    msg: &[u8],
    sig: &Signature,
) -> bool {
    pubkey.verify(msg, sig).is_ok()
}