use aes::Aes128;
use base32::{decode, encode, Alphabet};
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc};
use crc::Crc;
use ed25519_dalek::{SigningKey, VerifyingKey};
use hex;
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap; // Required for signing operations

// use aes_gcm::aead::generic_array::GenericArray;
// use aes_gcm::{
//     aead::{Aead, OsRng},
//     Aes256Gcm, KeyInit,
// };
// use bip39::{Language, Mnemonic, Seed};
// use std::fs;
// use std::io;
// use std::path::Path;

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

/// Stellar StrKey version bytes
const PUBLIC_KEY_VERSION_BYTE: u8 = 6 << 3; // 48 (0x30)
const SECRET_KEY_VERSION_BYTE: u8 = 18 << 3; // 144 (0x90)

#[derive(Serialize, Deserialize)]
pub struct Keystore {
    keys: HashMap<String, String>, // Public Key -> Secret Key
}

#[derive(Serialize, Deserialize)]
pub struct Identity {
    pub name: String,
    pub seed_phrase: String,
}

// impl Identity {
//     /// Generate a new identity with a seed phrase.
//     ///
//     /// # Arguments
//     /// * `name` - Name of the identity.
//     /// * `seed` - Optional seed to use when generating the seed phrase. If empty, a random seed is generated.
//     ///
//     /// # Returns
//     /// A tuple containing the seed phrase and the derived Stellar keypair.
//     pub fn generate(name: String, seed: Option<String>) -> (String, KeyPair) {
//         let mnemonic = match seed {
//             Some(seed) => Mnemonic::from_phrase(&seed, Language::English).unwrap(),
//             None => Mnemonic::generate_in(Language::English, 24).unwrap(),
//         };

//         let seed_phrase = mnemonic.phrase();
//         let seed_bytes = Seed::new(&mnemonic, "").as_bytes();
//         let keypair = KeyPair::from_seed(&seed_bytes[..32]).unwrap();

//         (seed_phrase.to_string(), keypair)
//     }

//     /// Save the identity to an encrypted file.
//     ///
//     /// # Arguments
//     /// * `name` - Name of the identity.
//     /// * `seed_phrase` - The seed phrase to save.
//     /// * `password` - Password to encrypt the file.
//     ///
//     /// # Returns
//     /// `io::Result<()>` indicating success or failure.
//     pub fn save(&self, password: &str) -> io::Result<()> {
//         let encrypted = encrypt(&serde_json::to_string(self).unwrap(), password);
//         fs::write(format!("{}.identity", self.name), encrypted)?;
//         Ok(())
//     }

//     /// Load an identity from an encrypted file.
//     ///
//     /// # Arguments
//     /// * `name` - Name of the identity.
//     /// * `password` - Password to decrypt the file.
//     ///
//     /// # Returns
//     /// `io::Result<Identity>` containing the loaded identity.
//     pub fn load(name: &str, password: &str) -> io::Result<Identity> {
//         let encrypted = fs::read_to_string(format!("{}.identity", name))?;
//         let decrypted = decrypt(&encrypted, password).unwrap();
//         let identity: Identity = serde_json::from_str(&decrypted)?;
//         Ok(identity)
//     }
// }

// /// Encrypt data using AES-256-GCM.
// fn encrypt(data: &str, password: &str) -> String {
//     let key = GenericArray::from_slice(password.as_bytes());
//     let cipher = Aes256Gcm::new(key);
//     let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
//     let ciphertext = cipher.encrypt(&nonce, data.as_bytes()).unwrap();
//     base64::encode(&[nonce.to_vec(), ciphertext].concat())
// }

// /// Decrypt data using AES-256-GCM.
// fn decrypt(encrypted: &str, password: &str) -> Result<String, Box<dyn std::error::Error>> {
//     let data = base64::decode(encrypted)?;
//     let (nonce, ciphertext) = data.split_at(12);
//     let key = GenericArray::from_slice(password.as_bytes());
//     let cipher = Aes256Gcm::new(key);
//     let plaintext = cipher.decrypt(GenericArray::from_slice(nonce), ciphertext)?;
//     Ok(String::from_utf8(plaintext)?)
// }

impl Keystore {
    pub fn new() -> Self {
        Keystore {
            keys: HashMap::new(),
        }
    }

    // Public getter for the number of keys
    pub fn key_count(&self) -> usize {
        self.keys.len()
    }

    // Public getter to retrieve all keys (optional, if needed)
    pub fn get_keys(&self) -> &HashMap<String, String> {
        &self.keys
    }

    pub fn add_key(&mut self, public_key: String, secret_key: String) {
        self.keys.insert(public_key, secret_key);
    }

    pub fn encrypt(&self, password: &str) -> String {
        // Hash the password to generate a 32-byte key
        let mut hasher = Sha256::new();
        hasher.update(password.as_bytes());
        let key = hasher.finalize();

        let iv = [0u8; 16]; // Initialization vector (IV)
        let cipher = Aes128Cbc::new_from_slices(&key[..16], &iv).unwrap();
        let serialized = serde_json::to_string(&self).unwrap();
        let ciphertext = cipher.encrypt_vec(serialized.as_bytes());
        hex::encode(ciphertext)
    }

    pub fn decrypt(password: &str, ciphertext: &str) -> Self {
        // Hash the password to generate a 32-byte key
        let mut hasher = Sha256::new();
        hasher.update(password.as_bytes());
        let key = hasher.finalize();

        let iv = [0u8; 16]; // Initialization vector (IV)
        let cipher = Aes128Cbc::new_from_slices(&key[..16], &iv).unwrap();
        let ciphertext = hex::decode(ciphertext).unwrap();
        let decrypted = cipher.decrypt_vec(&ciphertext).unwrap();
        serde_json::from_slice(&decrypted).unwrap()
    }

    /// Compute the CRC16-XModem checksum for Stellar StrKey.
    fn compute_checksum(data: &[u8]) -> [u8; 2] {
        // Use the predefined CRC16-XModem algorithm from the `crc` crate
        const CRC16_XMODEM: Crc<u16> = Crc::<u16>::new(&crc::CRC_16_XMODEM);

        // Compute the checksum
        let checksum = CRC16_XMODEM.checksum(data);

        // Return the checksum as a byte array
        [(checksum >> 8) as u8, (checksum & 0xFF) as u8]
    }

    /// Encode raw bytes into Stellar StrKey format.
    fn encode_stellar_key(data: &[u8], version_byte: u8) -> String {
        // Step 1: Prepend the version byte
        let mut payload = Vec::with_capacity(data.len() + 1);
        payload.push(version_byte);
        payload.extend_from_slice(data);

        // Step 2: Compute the checksum
        let checksum = Self::compute_checksum(&payload);

        // Step 3: Append the checksum to the payload
        payload.extend_from_slice(&checksum);

        // Step 4: Encode the payload using Base32
        encode(Alphabet::RFC4648 { padding: false }, &payload)
    }

    /// Decode a Stellar StrKey into raw bytes.
    pub fn decode_stellar_key(key: &str) -> Result<Vec<u8>, String> {
        // Step 1: Decode the Base32 string
        let decoded =
            decode(Alphabet::RFC4648 { padding: false }, key).ok_or("Invalid Base32 encoding")?; // Fail early if decoding fails

        // Step 2: Verify the checksum
        if decoded.len() < 3 {
            return Err("Decoded data is too short".to_string());
        }

        let payload = &decoded[..decoded.len() - 2];
        let checksum = &decoded[decoded.len() - 2..];

        let expected_checksum = Self::compute_checksum(payload);
        if checksum != expected_checksum {
            return Err("Checksum verification failed".to_string());
        }

        // Step 3: Return the raw key bytes (excluding the version byte)
        Ok(payload[1..].to_vec())
    }

    pub fn generate_keypair() -> (String, String) {
        let mut csprng = OsRng;
        let signing_key: SigningKey = SigningKey::generate(&mut csprng);
        // let signing_key: SigningKey = SigningKey::from_bytes
        let verifying_key: VerifyingKey = signing_key.verifying_key();

        let public_key_bytes = verifying_key.as_bytes();
        let secret_key_bytes = signing_key.to_bytes();

        // Encode keys in Stellar StrKey format
        let public_key = Self::encode_stellar_key(public_key_bytes, PUBLIC_KEY_VERSION_BYTE);
        let secret_key = Self::encode_stellar_key(&secret_key_bytes, SECRET_KEY_VERSION_BYTE);

        (public_key, secret_key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_keypair() {
        // Generate a keypair
        let (public_key, secret_key) = Keystore::generate_keypair();

        // Ensure the public key starts with 'G' and the secret key starts with 'S'
        assert!(public_key.starts_with('G'));
        assert!(secret_key.starts_with('S'));

        // Decode the public key and verify its length
        let decoded_public_key = Keystore::decode_stellar_key(&public_key).unwrap();
        assert_eq!(decoded_public_key.len(), 32);

        // Decode the secret key and verify its length
        let decoded_secret_key = Keystore::decode_stellar_key(&secret_key).unwrap();
        assert_eq!(decoded_secret_key.len(), 32);
    }

    #[test]
    fn test_encode_decode_stellar_key() {
        // Example raw public key bytes
        let public_key_bytes = [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
            0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C,
            0x1D, 0x1E, 0x1F, 0x20,
        ];

        // Encode the public key
        let encoded_public_key =
            Keystore::encode_stellar_key(&public_key_bytes, PUBLIC_KEY_VERSION_BYTE);

        // Decode the public key
        let decoded_public_key = Keystore::decode_stellar_key(&encoded_public_key).unwrap();

        // Verify the decoded bytes match the original
        assert_eq!(decoded_public_key, public_key_bytes);
    }

    #[test]
    fn test_encrypt_decrypt_keystore() {
        // Create a new keystore
        let mut keystore = Keystore::new();
        keystore.add_key(
            "GCKFLO7RHFB2HXMEN6EMZPT6WYCIOL3WJPDJWGIKBQA2KAVS6VS4WY4E".to_string(),
            "SB3KUBH6VZ7ZL7ZVJ4XUQY5Z6Z7ZL7ZVJ4XUQY5Z6Z7ZL7ZVJ4XUQY5Z6".to_string(),
        );

        // Encrypt the keystore
        let password = "mypassword";
        let encrypted = keystore.encrypt(password);

        // Decrypt the keystore
        let decrypted_keystore = Keystore::decrypt(password, &encrypted);

        // Verify the decrypted keystore matches the original
        assert_eq!(decrypted_keystore.keys.len(), 1);
        assert!(decrypted_keystore
            .keys
            .contains_key("GCKFLO7RHFB2HXMEN6EMZPT6WYCIOL3WJPDJWGIKBQA2KAVS6VS4WY4E"));
    }

    #[test]
    fn test_checksum_verification() {
        // Example raw public key bytes
        let public_key_bytes = [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
            0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C,
            0x1D, 0x1E, 0x1F, 0x20,
        ];

        // Compute the checksum
        let payload = {
            let mut temp = vec![PUBLIC_KEY_VERSION_BYTE];
            temp.extend_from_slice(&public_key_bytes);
            temp
        };
        let checksum = Keystore::compute_checksum(&payload);

        // Verify the checksum computation
        const CRC16_XMODEM: Crc<u16> = Crc::<u16>::new(&crc::CRC_16_XMODEM);
        let expected_checksum = CRC16_XMODEM.checksum(&payload);
        assert_eq!(
            checksum,
            [
                (expected_checksum >> 8) as u8,
                (expected_checksum & 0xFF) as u8
            ]
        );
    }

    #[test]
    fn test_decode_stellar_key_decoded_data_too_short() {
        // Base32-decoded data is too short (less than 3 bytes)
        let short_data = encode(Alphabet::RFC4648 { padding: false }, &[0x01]);
        let result = Keystore::decode_stellar_key(&short_data);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Decoded data is too short");
    }

    #[test]
    fn test_decode_stellar_key_checksum_failure() {
        // Corrupt the checksum to simulate a failure
        let public_key_bytes = [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
            0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C,
            0x1D, 0x1E, 0x1F, 0x20,
        ];
        let mut payload = vec![PUBLIC_KEY_VERSION_BYTE];
        payload.extend_from_slice(&public_key_bytes);
        let checksum = Keystore::compute_checksum(&payload);
        payload.extend_from_slice(&checksum);

        // Corrupt the checksum
        let len = payload.len();
        payload[len - 1] ^= 0xFF;

        // Encode the corrupted payload
        let invalid_key = encode(Alphabet::RFC4648 { padding: false }, &payload);

        // Attempt to decode the invalid key
        let result = Keystore::decode_stellar_key(&invalid_key);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Checksum verification failed");
    }

    #[test]
    fn test_decode_stellar_key_valid_key() {
        // Valid Stellar public key
        let public_key_bytes = [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
            0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C,
            0x1D, 0x1E, 0x1F, 0x20,
        ];
        let mut payload = vec![PUBLIC_KEY_VERSION_BYTE];
        payload.extend_from_slice(&public_key_bytes);
        let checksum = Keystore::compute_checksum(&payload);
        payload.extend_from_slice(&checksum);

        // Encode the valid payload
        let valid_key = encode(Alphabet::RFC4648 { padding: false }, &payload);

        // Decode the valid key
        let result = Keystore::decode_stellar_key(&valid_key);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), public_key_bytes);
    }
}
