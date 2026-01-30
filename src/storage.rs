use argon2::{
    Algorithm, Argon2, Params, Version,
    password_hash::{
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::RngCore,
    },
};
use chacha20poly1305::{
    AeadCore, XChaCha20Poly1305, XNonce,
    aead::{Aead, KeyInit, OsRng},
};
use std::collections::HashMap;
use std::fs;

pub fn get_or_create_salt(path: &str) -> String {
    if std::path::Path::new(path).exists() {
        fs::read_to_string(path).expect("Failed to read salt file")
    } else {
        let mut salt_bytes = [0u8; 16];
        let mut rng = chacha20poly1305::aead::OsRng;
        rng.fill_bytes(&mut salt_bytes);
        let new_salt = hex::encode(salt_bytes);
        fs::write(path, &new_salt).expect("Failed to save salt file");
        new_salt
    }
}

pub fn derive_key(password: &str, salt: &str) -> [u8; 32] {
    let mut key = [0u8; 32];
    let argon2 = Argon2::default();

    let salt_string =
        SaltString::encode_b64(salt.as_bytes()).expect("Salt string is too long or invalid b64");

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt_string)
        .expect("Failed to hash password");

    let output = password_hash.hash.expect("Hash output missing");

    let output_bytes = output.as_bytes();
    if output_bytes.len() >= 32 {
        key.copy_from_slice(&output_bytes[..32]);
    } else {
        panic!("Argon2 output too short! We need 32 bytes.");
    }

    key
}

pub fn load_passwords(path: &str, key: &[u8; 32]) -> HashMap<String, String> {
    let data = match fs::read(path) {
        Ok(d) => d,
        Err(_) => return HashMap::new(),
    };
    if data.len() < 24 {
        return HashMap::new();
    }

    let (nonce_bytes, ciphertext) = data.split_at(24);
    let nonce = XNonce::from_slice(nonce_bytes);

    let cipher = XChaCha20Poly1305::new(key.into());
    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .expect("Decryption failed! Data corrupted or wrong key.");

    let json_string = String::from_utf8(plaintext).expect("Invalid UTF-8");
    serde_json::from_str(&json_string).unwrap_or_default()
}

pub fn save_passwords(path: &str, data: &HashMap<String, String>, key: &[u8; 32]) {
    let json = serde_json::to_string(data).expect("Failed to serialize");

    let cipher = XChaCha20Poly1305::new(key.into());
    let nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng);

    let ciphertext = cipher
        .encrypt(&nonce, json.as_bytes())
        .expect("Encryption failed");

    let mut combined = nonce.to_vec();
    combined.extend(ciphertext);
    fs::write(path, combined).expect("Failed to write to file");
}

pub fn delete_password(path: &str, name: &str, key: &[u8; 32]) {
    let mut passwords = load_passwords(path, key);
    if passwords.remove(name).is_some() {
        save_passwords(path, &passwords, key);
        println!("Successfully deleted password for: {}", name);
    } else {
        println!("Error: No password found with the name '{}'", name);
    }
}

pub fn hash_master_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let params = Params::new(131072, 5, 4, None).expect("Invalid params");
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::default(), params);

    argon2
        .hash_password(password.as_bytes(), &salt)
        .expect("Error hashing password")
        .to_string()
}

pub fn verify_master_password(password: &str, recorded_hash: &str) -> bool {
    let argon2 = Argon2::default();
    let parsed_hash = PasswordHash::new(recorded_hash).expect("Invalid hash format");

    argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}
