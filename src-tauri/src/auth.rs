use aes_gcm::{
    aead::{generic_array::typenum::U12, Aead, AeadCore, KeyInit},
    Aes256Gcm, Key, Nonce,
};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use base64::{engine::general_purpose, Engine as _};
use core::str;

// Function to generate a random salt (done once)
fn generate_salt() -> SaltString {
    SaltString::generate(&mut OsRng)
}

// Function to hash the password with a newly generated salt
#[tauri::command]
pub(crate) fn hash_password(password: String) -> String {
    let argon2 = Argon2::default();

    // Generate a new salt using the internal generate_salt function
    let salt = generate_salt();

    // Hash the password with the generated salt
    match argon2.hash_password(password.as_bytes(), &salt) {
        Ok(password_hash) => password_hash.to_string(),
        Err(e) => format!("Error hashing password: {}", e.to_string()),
    }
}

// Function to verify the password with the stored hash
#[tauri::command]
pub(crate) fn verify_password(hash: String, password: String) -> Result<bool, String> {
    let parsed_hash = PasswordHash::new(&hash).map_err(|e| e.to_string())?; // Parse the hash string
    let verify_result = Argon2::default().verify_password(password.as_bytes(), &parsed_hash);

    // Return true if the password verification succeeded, otherwise propagate the error
    match verify_result {
        Ok(_) => Ok(true),
        Err(e) => Err(e.to_string()), // Convert error to a string and return
    }
}

fn derive_key_from_password(master_password: String, salt: &str) -> Vec<u8> {
    let argon2 = Argon2::default();
    let mut derived_key = vec![0u8; 32];

    // Derive the key
    argon2
        .hash_password_into(
            master_password.as_bytes(),
            salt.as_bytes(),
            &mut derived_key,
        )
        .expect("Error deriving key");

    derived_key
}

pub(crate) fn encrypt_password(
    master_password: String,
    password: String,
) -> (String, String, String) {
    let binding = generate_salt();
    let salt = binding.as_ref();
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // Generates a 12-byte nonce for AES-GCM

    let key = derive_key_from_password(master_password, salt);
    let key = Key::<Aes256Gcm>::from_slice(&key);

    let cipher = Aes256Gcm::new(&key);

    let ciphertext = cipher
        .encrypt(Nonce::from_slice(&nonce), password.as_bytes())
        .expect("Encryption failed");

    let b64_salt = general_purpose::STANDARD.encode(&salt);
    let b64_nonce = general_purpose::STANDARD.encode(&nonce);
    let b64_ciphertext = general_purpose::STANDARD.encode(&ciphertext);

    (b64_salt, b64_nonce, b64_ciphertext)
}

pub(crate) fn decrypt_password(
    master_password: &String,
    salt: String,
    nonce: String,
    ciphertext: String,
) -> Result<String, String> {
    // Decode components, returning general errors on failure
    let decoded_salt = general_purpose::STANDARD
        .decode(salt)
        .map_err(|_| "Failed to decode salt")?;
    let decoded_nonce = general_purpose::STANDARD
        .decode(nonce)
        .map_err(|_| "Failed to decode nonce")?;
    let decoded_ciphertext = general_purpose::STANDARD
        .decode(ciphertext)
        .map_err(|_| "Failed to decode ciphertext")?;
    let decoded_salt = str::from_utf8(&decoded_salt).map_err(|_| "Failed to convert salt to string")?;

    // Derive the encryption key from the master password and salt
    let key = derive_key_from_password(master_password.to_string(), decoded_salt);
    let key = Key::<Aes256Gcm>::from_slice(&key);

    // Initialize AES-GCM with the derived key
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::<U12>::from_slice(&decoded_nonce);

    // Decrypt the ciphertext
    let plaintext = cipher
        .decrypt(&nonce, decoded_ciphertext.as_ref())
        .map_err(|_| "Decryption failed")?;

    // Convert plaintext bytes to a UTF-8 string
    let plaintext =
        String::from_utf8(plaintext).map_err(|_| "Failed to convert plaintext to string")?;

    Ok(plaintext)
}
