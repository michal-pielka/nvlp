use std::io::{Read, Write};

use age::armor::Format::AsciiArmor;
use age::armor::{ArmoredReader, ArmoredWriter};
use age::ssh::{Identity as SshIdentity, Recipient as SshRecipient};
use age::{Decryptor, Encryptor, Identity, Recipient};

use crate::error::{Error, Result};

pub fn encrypt(plaintext: &[u8], public_keys: &[&str]) -> Result<Vec<u8>> {
    let recipients = parse_recipients(public_keys)?;
    let recipients = recipients.iter().map(|r| r as &dyn Recipient);

    let encryptor =
        Encryptor::with_recipients(recipients).map_err(|e| Error::Encrypt(e.to_string()))?;
    let mut ciphertext = Vec::new();

    let armored = ArmoredWriter::wrap_output(&mut ciphertext, AsciiArmor)
        .map_err(|e| Error::Encrypt(e.to_string()))?;
    let mut writer = encryptor
        .wrap_output(armored)
        .map_err(|e| Error::Encrypt(e.to_string()))?;
    writer.write_all(plaintext)?;
    let armored = writer.finish()?;
    armored.finish()?;

    Ok(ciphertext)
}

pub fn decrypt(
    ciphertext: &[u8],
    private_key: &str,
    private_key_filename: Option<&str>,
) -> Result<Vec<u8>> {
    let identity = parse_identity(private_key, private_key_filename)?;

    let armored = ArmoredReader::new(ciphertext);
    let decryptor = Decryptor::new(armored).map_err(|e| Error::Decrypt(e.to_string()))?;
    let mut plaintext = Vec::new();

    let mut reader = decryptor
        .decrypt(std::iter::once(&identity as &dyn Identity))
        .map_err(|e| Error::Decrypt(e.to_string()))?;
    reader.read_to_end(&mut plaintext)?;

    Ok(plaintext)
}

fn parse_recipients(public_keys: &[&str]) -> Result<Vec<SshRecipient>> {
    public_keys
        .iter()
        .map(|&key| {
            key.parse::<SshRecipient>()
                .map_err(|e| Error::KeyParse(format!("{:?}", e)))
        })
        .collect()
}

fn parse_identity(private_key: &str, private_key_filename: Option<&str>) -> Result<SshIdentity> {
    SshIdentity::from_buffer(
        private_key.as_bytes(),
        private_key_filename.map(|f| f.to_string()),
    )
    .map_err(|e| Error::KeyParse(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn roundtrip(public_key: &str, private_key: &str) {
        let plaintext = b"test plaintext for roundtrip";
        let encrypted = encrypt(plaintext, &[public_key]).unwrap();
        let decrypted = decrypt(&encrypted, private_key, None).unwrap();
        assert_eq!(plaintext.as_ref(), &decrypted);
    }

    #[test]
    fn test_ed25519_roundtrip() {
        roundtrip(
            include_str!("testdata/test_id_ed25519.pub"),
            include_str!("testdata/test_id_ed25519"),
        );
    }

    #[test]
    fn test_rsa_roundtrip() {
        roundtrip(
            include_str!("testdata/test_rsa.pub"),
            include_str!("testdata/test_rsa"),
        );
    }
}
