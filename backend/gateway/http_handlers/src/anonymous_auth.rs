use domain_model::UserId;
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// サーバーの秘密鍵で匿名User IDを署名・検証する。
#[derive(Clone)]
pub struct AnonymousAuth {
    secret: Vec<u8>,
}

impl AnonymousAuth {
    /// 推測されにくい32バイト以上の秘密鍵を必須とする。
    pub fn new(secret: String) -> Result<Self, &'static str> {
        if secret.len() < 32 {
            return Err("ANONYMOUS_AUTH_SECRET must contain at least 32 bytes");
        }
        Ok(Self {
            secret: secret.into_bytes(),
        })
    }

    /// 新しい匿名Userに、IDと署名を含むBearer tokenを発行する。
    pub fn issue(&self) -> (UserId, String) {
        let user_id = UserId::generate();
        let id = user_id.to_string();
        let mut mac = HmacSha256::new_from_slice(&self.secret).expect("HMAC accepts any key size");
        mac.update(id.as_bytes());
        let signature = hex::encode(mac.finalize().into_bytes());
        (user_id, format!("{id}.{signature}"))
    }

    /// 署名が正しいBearer tokenから所有者IDを復元する。
    pub fn verify(&self, token: &str) -> Option<UserId> {
        let (id, signature) = token.split_once('.')?;
        let user_id = UserId::try_from(id.to_owned()).ok()?;
        let bytes = hex::decode(signature).ok()?;
        let mut mac = HmacSha256::new_from_slice(&self.secret).ok()?;
        mac.update(id.as_bytes());
        mac.verify_slice(&bytes).ok()?;
        Some(user_id)
    }
}

#[cfg(test)]
mod tests {
    use super::AnonymousAuth;

    /// 発行済みtokenだけが検証され、IDや署名の改変を拒否する。
    #[test]
    fn verifies_only_issued_tokens() {
        let auth = AnonymousAuth::new("test-secret-with-at-least-32-bytes-long".into()).unwrap();
        let (user_id, token) = auth.issue();
        assert_eq!(auth.verify(&token), Some(user_id));
        assert_eq!(auth.verify(&user_id.to_string()), None);
        let (other_id, _) = auth.issue();
        let (_, signature) = token.split_once('.').unwrap();
        assert_eq!(auth.verify(&format!("{other_id}.{signature}")), None);
        assert_eq!(auth.verify(&format!("{user_id}.bad")), None);
    }
}
