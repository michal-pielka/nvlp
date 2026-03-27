use std::io::{Read, Write};

use age::ssh::{Identity as SshIdentity, ParseRecipientKeyError, Recipient as SshRecipient};
use age::{Decryptor, Encryptor, Identity, Recipient};

pub fn encrypt(
    plaintext: &[u8],
    public_keys: &[&str],
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let recipients = parse_recipients(public_keys).map_err(|_e| "TODO: Custom error")?;
    let recipients = recipients.iter().map(|r| r as &dyn Recipient);

    let encryptor = Encryptor::with_recipients(recipients)?;
    let mut ciphertext = Vec::new();

    let mut writer = encryptor.wrap_output(&mut ciphertext)?;
    writer.write_all(plaintext)?;
    writer.finish()?;

    Ok(ciphertext)
}

pub fn decrypt(
    ciphertext: &[u8],
    private_key: &str,
    private_key_filename: Option<&str>,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let identity = parse_identity(private_key, private_key_filename).map_err(|_e| "TODO: Custom error")?;

    let decryptor = Decryptor::new(ciphertext)?;
    let mut plaintext = Vec::new();

    let mut reader = decryptor.decrypt(std::iter::once(&identity as &dyn Identity))?;
    reader.read_to_end(&mut plaintext)?;

    Ok(plaintext)
}

fn parse_recipients(public_keys: &[&str]) -> Result<Vec<SshRecipient>, ParseRecipientKeyError> {
    public_keys
        .iter()
        .map(|&key| key.parse::<SshRecipient>())
        .collect()
}

fn parse_identity(private_key: &str, private_key_filename: Option<&str>) -> Result<SshIdentity, std::io::Error> {
    SshIdentity::from_buffer(private_key.as_bytes(), private_key_filename.map(|f| f.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt_ed25519_roundtrip() {
        let plaintext = "plaintext soon to be encrypted with ed25519 key";
        let ed25519_private_key = include_str!("testdata/test_id_ed25519");
        let ed25519_public_key = include_str!("testdata/test_id_ed25519.pub");

        let encrypted = encrypt(plaintext.as_bytes(), &[ed25519_public_key]).unwrap();
        let decrypted = decrypt(&encrypted, ed25519_private_key, None).unwrap();

        assert_eq!(plaintext.as_bytes(), &decrypted);
    }

    #[test]
    fn test_encrypt_decrypt_rsa_roundtrip() {
        let plaintext = "plaintext soon to be encrypted with rsa key";
        let rsa_private_key = include_str!("testdata/test_rsa");
        let rsa_public_key = include_str!("testdata/test_rsa.pub");

        let encrypted = encrypt(plaintext.as_bytes(), &[rsa_public_key]).unwrap();
        let decrypted = decrypt(&encrypted, rsa_private_key, None).unwrap();

        assert_eq!(plaintext.as_bytes(), &decrypted);
    }
}
