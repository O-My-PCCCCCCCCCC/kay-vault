use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use argon2::Argon2;
use password_hash::{PasswordHasher, SaltString};
use rand::RngCore;

/// 使用 Argon2id 从主密码派生 256-bit 加密密钥
pub fn derive_key(password: &str, salt: &[u8]) -> Vec<u8> {
    let salt_str = SaltString::encode_b64(salt).expect("无效 salt");
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt_str)
        .expect("密钥派生失败");
    hash.hash.unwrap().as_bytes().to_vec()
}

/// 生成随机 32 字节 salt
pub fn generate_salt() -> Vec<u8> {
    let mut salt = vec![0u8; 32];
    OsRng.fill_bytes(&mut salt);
    salt
}

/// AES-256-GCM 加密：返回 [nonce(12B) + ciphertext + tag(16B)]
pub fn encrypt(plaintext: &[u8], key: &[u8]) -> Vec<u8> {
    let key_arr = aes_gcm::Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(key_arr);

    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher.encrypt(nonce, plaintext).expect("加密失败");

    let mut result = Vec::with_capacity(12 + ciphertext.len());
    result.extend_from_slice(&nonce_bytes);
    result.extend_from_slice(&ciphertext);
    result
}

/// AES-256-GCM 解密
pub fn decrypt(data: &[u8], key: &[u8]) -> Result<Vec<u8>, String> {
    if data.len() < 12 {
        return Err("数据太短".into());
    }
    let (nonce_bytes, ciphertext) = data.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);

    let key_arr = aes_gcm::Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(key_arr);

    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| format!("解密失败: {:?}", e))
}
