//! Argon2id password hashing. The PHC-format string returned by `hash`
//! embeds the salt and parameters, so it's the only thing we store.

use anyhow::{anyhow, Result};
use argon2::Argon2;
use password_hash::{
    rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
};

/// Hash a password using Argon2id with a fresh per-user salt. Returns the
/// canonical PHC string suitable for storage in `account.password_hash`.
pub fn hash(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let phc = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow!("argon2 hash failed: {e}"))?
        .to_string();
    Ok(phc)
}

/// Verify a candidate password against a stored PHC hash. Returns false
/// when the password is wrong; returns `Err` only if the stored hash is
/// malformed (a data-corruption symptom, not a normal auth failure).
pub fn verify(password: &str, stored: &str) -> Result<bool> {
    let parsed = PasswordHash::new(stored)
        .map_err(|e| anyhow!("stored hash is malformed: {e}"))?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_verifies() {
        let hash = hash("correct horse battery staple").unwrap();
        assert!(verify("correct horse battery staple", &hash).unwrap());
    }

    #[test]
    fn wrong_password_fails() {
        let hash = hash("correct horse battery staple").unwrap();
        assert!(!verify("nope", &hash).unwrap());
    }

    #[test]
    fn each_hash_uses_a_fresh_salt() {
        // Hashing the same password twice must produce different PHC strings.
        let a = hash("same password").unwrap();
        let b = hash("same password").unwrap();
        assert_ne!(a, b);
        assert!(verify("same password", &a).unwrap());
        assert!(verify("same password", &b).unwrap());
    }

    #[test]
    fn malformed_hash_errors() {
        assert!(verify("anything", "not a real hash").is_err());
    }
}
