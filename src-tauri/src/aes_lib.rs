use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use rand_core::{OsRng, RngCore};

type Aes128CbcEnc = cbc::Encryptor<aes::Aes128>;
type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

const KEY: &[u8; 16] = b"abcdedghijklmnop"; // 模拟密钥，请勿在实际程序中使用

/// 生成随机 iv
fn generate_iv() -> [u8; 16] {
    let mut rng = OsRng;
    let mut bytes = [0u8; 16];
    rng.fill_bytes(&mut bytes);

    bytes
}

/// 加密
pub fn encrypt(plain: &[u8]) -> (Vec<u8>, [u8; 16]) {
    // 随机值
    let iv = generate_iv();

    let mut buf = [0u8; 1024];
    let pt_len = plain.len();
    buf[..pt_len].copy_from_slice(plain);
    let ct = Aes128CbcEnc::new(KEY.into(), &iv.into())
        .encrypt_padded_b2b_mut::<Pkcs7>(plain, &mut buf)
        .unwrap();

    (ct.to_vec(), iv)
}

/// 解密
pub fn decrypt(cipher: &[u8], iv: [u8; 16]) -> Vec<u8> {
    let cipher_len = cipher.len();
    let mut buf = [0u8; 1024];
    buf[..cipher_len].copy_from_slice(cipher);

    let pt = Aes128CbcDec::new(KEY.into(), &iv.into())
        .decrypt_padded_b2b_mut::<Pkcs7>(cipher, &mut buf)
        .unwrap();

    pt.to_vec()
}

#[test]
fn aes_test() {
    // 账号密码应为单向加密，参考：https://github.com/RustCrypto/password-hashes
    // 这里的示例代码应用来加密如手机号、身份证号、银行卡号等涉及用户隐私的数据
    let separator = "*".repeat(40);

    let plain = b"This is not a password \xE6\x88\x91";
    println!("明文：{:?}", plain);
    let (ct, iv) = encrypt(plain);
    println!(
        "{}\n密文：{:?}\n初始化向量：{:?}\n{}",
        separator, ct, iv, separator
    );
    let pt = decrypt(&ct, iv);
    println!("解密结果：{:?}", pt);

    assert_eq!(plain.to_vec(), pt);
}