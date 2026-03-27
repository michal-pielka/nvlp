use std::io::Write;

use age::ssh::{ParseRecipientKeyError, Recipient as SshRecipient};
use age::{Encryptor, Recipient};

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
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    Ok(vec![])
}

fn parse_recipients(public_keys: &[&str]) -> Result<Vec<SshRecipient>, ParseRecipientKeyError> {
    public_keys
        .iter()
        .map(|&key| key.parse::<SshRecipient>())
        .collect()
}

fn parse_identity(private_key: &str) -> Result<SshIdentity, std::io::Error> {
    SshIdentity::from_buffer(private_key.as_bytes(), None)
}
