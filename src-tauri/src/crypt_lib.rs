use base64::{engine::general_purpose, Engine as _};
/// 加密
pub fn encrypt(src: &str) -> String {
    let key: &str = "TcEPse6";
    general_purpose::STANDARD
        .encode(key.to_owned() + &general_purpose::STANDARD.encode(key.to_owned() + src))
}

/// 解密
pub fn decrypt(src: &str) -> String {
    let mut out: String = "".to_string();
    if let Ok(dst_bytes) = general_purpose::STANDARD.decode(src) {
        if let Ok(dst_str1) = String::from_utf8(dst_bytes) {
            let (_, dst) = dst_str1.split_at(7);
            if let Ok(dst_bytes2) = general_purpose::STANDARD.decode(dst) {
                if let Ok(dst_str1) = String::from_utf8(dst_bytes2) {
                    let (_, dst) = dst_str1.split_at(7);
                    out = dst.to_string();
                }
            }
        }
    }
    out
}

#[test]
fn aes_test() {
    let src = "2234我是wewqewqeeq";
    let a = encrypt(&src);
    println!("{}", &a);

    let dst = decrypt(&a);

    println!("{}", &dst);

    assert_eq!(src.to_string(), dst);
}
