use age::ssh::{ParseRecipientKeyError, Recipient};
pub fn encrypt(plaintext: &[u8], public_keys: &[&str]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    Ok(vec![])
}

pub fn decrypt(ciphertext: &[u8], private_key: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    Ok(vec![])
}
fn parse_recipients(public_keys: &[&str]) -> Result<Vec<Recipient>, ParseRecipientKeyError> {
    public_keys
        .iter()
        .map(|&key| key.parse::<Recipient>())
        .collect()
}
