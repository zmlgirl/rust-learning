use rsa::pkcs8::{DecodePublicKey, DecodePrivateKey};

fn main() -> anyhow::Result<()> {
    use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};

    let mut rng = rand::thread_rng();
    let bits = 1024;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);

    // 公钥加密 Encrypt
    let encypt_data = "hello world";
    let enc_data = pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, encypt_data.as_bytes()).expect("failed to encrypt");

    // 私钥解密 Decrypt
    let dec_data = priv_key.decrypt(Pkcs1v15Encrypt, &enc_data).expect("failed to decrypt");
    println!("decryptd data: {}", String::from_utf8(dec_data)?);
    // assert_eq!(data[..], &dec_data[..]);

    // 使用公钥密钥明文进行加解密
    let pub_key_pem = "-----BEGIN PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQC//nB6rRTnxCU2bMBGatp1N1Q0
kuSEZl3Ot2EQMlNwINYTm7izxjTyA1pgmBmotAXVZuZNviJNUZUMBn73bIjso1l2
qhwe/FcewPjP2ubbdf89yWPnen/wRGo+Q0QRmt1q7eDeVTJMC4LVdetuv6QABnUJ
+siG1ILDsJ2BsYMBMwIDAQAB
-----END PUBLIC KEY-----";
    let pri_key_pem = "-----BEGIN PRIVATE KEY-----
MIICdgIBADANBgkqhkiG9w0BAQEFAASCAmAwggJcAgEAAoGBAL/+cHqtFOfEJTZs
wEZq2nU3VDSS5IRmXc63YRAyU3Ag1hObuLPGNPIDWmCYGai0BdVm5k2+Ik1RlQwG
fvdsiOyjWXaqHB78Vx7A+M/a5tt1/z3JY+d6f/BEaj5DRBGa3Wrt4N5VMkwLgtV1
626/pAAGdQn6yIbUgsOwnYGxgwEzAgMBAAECgYArffoA8EByGC7N22jbCs2eDACZ
QEVf8MiDUWs9fgkEt4uHOZlnsBjCUhwhEZOOcel4ZCz7o3ylwzteVAJjDkUd4P5a
BI0qRwY4yKM0pMzGh4iEbZDslt+UhszOdiUln5pAhCoyjwY20Q8nTnRA1LHegUqy
KyDH7hmcyLnKOFxDkQJBAPdtUcUk4kffturW1ff9lucgz0dDlkwmrNvpSKl8hAWy
DllraR/xVmBP3j0+moMrxW8n8kkZHyC2HIlr243SqzUCQQDGpW1wuQrDkDSSx2yx
4VASURpbi8b1grFqnnV2rZaJV6Qz/DjeVQi35BYypbyqOyGIfD8DRBi2A7bN9xK0
Ap/HAkBD/K3zVec3iKlibhXtRqkoaUOLeW7bDeWBp+Bnced1CTAYMgN423+4hzmx
6nnagTSHDprsqxJ6ko/U0uZJWhHhAkEAw58Hjnl7oyjwzRb88JOdAdzCoyKcdQwW
YZPYw12hIHGhb9xTuIdvBYDlZ00V5WJD3J/WggXee+hebqiAz6rB4wJALqx4a5vZ
ZeL42kq2UeHejlgRjVKroqHJf/Ekq4SkSl4Icc4dLP7IibASL53Ar/KuSQ3RefXq
TCK6NNUxaRTKvQ==
-----END PRIVATE KEY-----";

    // 公钥加密 私钥解密
    let data = "hell world";
    let public_key = RsaPublicKey::from_public_key_pem(pub_key_pem)?;
    let encrypted_data = public_key.encrypt(&mut rng, Pkcs1v15Encrypt, data.as_bytes())?;
    // println!("encrypted data: {}", String::from_utf8(encrypted_data.clone())?);
    let encrypted_data = "U2FsdGVkX1/5y5p6zh2q6Z/yZv1WeXES2WlabnuAOWs=".as_bytes();
    let private_key = RsaPrivateKey::from_pkcs8_pem(pri_key_pem)?;
    let decrypted_data = private_key.decrypt(Pkcs1v15Encrypt, &encrypted_data)?;
    println!("decrypted_data: {}", String::from_utf8(decrypted_data)?);

    Ok(())
}