use argon2::{Argon2, PasswordHasher, password_hash::SaltString};
// use argon2::{Argon2, PasswordHasher, PasswordVerifier, password_hash::{SaltString, PasswordHash}};
use rand::thread_rng;

/// Hashes a password using Argon2.
pub fn hash_password(password: &str) -> String {
    // Generate a random salt
    let salt = SaltString::generate(&mut thread_rng());

    // Initialize the Argon2 hasher with default settings
    let argon2 = Argon2::default();

    // Hash the password
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)
        .expect("Failed to hash password");

    // Return the hashed password as a string
    password_hash.to_string()
}

// Verifies a plaintext password against a hashed password.
// pub fn verify_password(password: &str, hashed_password: &str) -> bool {
//     // Parse the stored hashed password into a PasswordHash object
//     let parsed_hash = PasswordHash::new(hashed_password)
//         .expect("Failed to parse stored password hash");

//     // Verify the password
//     let argon2 = Argon2::default();
//     argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok()
// }