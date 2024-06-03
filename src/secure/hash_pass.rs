use crate::Result;
use argon2::{self, Config, Variant, Version};

/// Computes the hashed value of a password using Argon2id algorithm.
///
/// # Arguments
///
/// * `password` - The password string to be hashed.
/// * `salt` - The salt string used in the hashing process.
///
/// # Returns
///
/// Returns the hashed value of the password as a String.
pub fn hash_password(password: &str, salt: &str) -> Result<String> {
    let config = Config {
        variant: Variant::Argon2id,
        version: Version::Version13,
        mem_cost: 65536,
        time_cost: 10,
        lanes: 4,
        secret: &[],
        ad: &[],
        hash_length: 32,
    };

    argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config).map_err(|e| e.into())
}

/// Verify a password against its hash.
///
/// This function takes a password and its corresponding hash and verifies
/// whether the password matches the hash. It makes use of the Argon2 password
/// hashing algorithm.
///
/// # Arguments
///
/// * `password` - The password to be verified as a string reference.
/// * `hash` - The hash of the password to be compared against, as a string reference.
///
/// # Returns
///
/// A boolean value indicating whether the password matches the provided hash.
///
/// # Panics
///
/// This function panics if the Argon2 library fails to verify the password
/// against the hash.
pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
    argon2::verify_encoded(hash, password.as_bytes()).map_err(|e| e.into())
}
