//! HS256 JWTs carrying the account id as `sub`. Tokens are stateless — no
//! server-side session store. Rotating `JWT_SECRET` invalidates every
//! outstanding token, which is the intended behaviour on compromise.

use std::time::Duration;

use anyhow::{anyhow, Result};
use jsonwebtoken::{
    decode, encode, errors::ErrorKind, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Claims {
    /// Subject — the account id.
    pub sub: Uuid,
    /// Issued-at, unix seconds.
    pub iat: i64,
    /// Expiry, unix seconds.
    pub exp: i64,
}

/// Issue a token for `account_id` valid for `ttl` from now.
pub fn issue(account_id: Uuid, secret: &str, ttl: Duration) -> Result<String> {
    let now = OffsetDateTime::now_utc().unix_timestamp();
    let claims = Claims {
        sub: account_id,
        iat: now,
        exp: now + ttl.as_secs() as i64,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| anyhow!("jwt encode failed: {e}"))
}

/// Decode and validate a token (signature + expiry). Returns the inner
/// claims. Distinguishes expiry from other malformed-token errors so the
/// caller can choose to surface `token_expired` separately if it ever
/// wants to; today the auth middleware collapses both to 401.
pub fn decode_token(token: &str, secret: &str) -> Result<Claims, DecodeError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|d| d.claims)
    .map_err(|e| match e.kind() {
        ErrorKind::ExpiredSignature => DecodeError::Expired,
        _ => DecodeError::Invalid,
    })
}

#[derive(Debug, PartialEq, Eq)]
pub enum DecodeError {
    Invalid,
    Expired,
}

#[cfg(test)]
mod tests {
    use super::*;

    const SECRET: &str = "test-secret-at-least-16-chars-long";

    #[test]
    fn round_trip() {
        let id = Uuid::new_v4();
        let token = issue(id, SECRET, Duration::from_secs(3600)).unwrap();
        let claims = decode_token(&token, SECRET).unwrap();
        assert_eq!(claims.sub, id);
        assert!(claims.exp > claims.iat);
    }

    #[test]
    fn wrong_secret_rejected() {
        let id = Uuid::new_v4();
        let token = issue(id, SECRET, Duration::from_secs(3600)).unwrap();
        assert_eq!(
            decode_token(&token, "different-secret-also-long-enough"),
            Err(DecodeError::Invalid),
        );
    }

    #[test]
    fn expired_token_is_distinguished() {
        // ttl=0 → exp == iat == now, immediately past.
        let token = issue(Uuid::new_v4(), SECRET, Duration::from_secs(0)).unwrap();
        // jsonwebtoken's Validation has a 60s leeway by default, which would
        // make an exp==now token still valid. Force-build a Validation with
        // zero leeway via the public test helper.
        let _ = token; // proof the issue path is exercised
        // Issue a token whose `exp` is well in the past by re-encoding by hand.
        let claims = Claims {
            sub: Uuid::new_v4(),
            iat: 0,
            exp: 1, // 1970-01-01 + 1s
        };
        let stale = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(SECRET.as_bytes()),
        )
        .unwrap();
        assert_eq!(decode_token(&stale, SECRET), Err(DecodeError::Expired));
    }

    #[test]
    fn garbage_token_rejected() {
        assert_eq!(
            decode_token("not.a.jwt", SECRET),
            Err(DecodeError::Invalid),
        );
    }
}
