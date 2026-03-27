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
