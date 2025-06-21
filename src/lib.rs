// Implement extract_tx_version function below
pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
    if raw_tx_hex.len() < 8 {
        return Err("Transaction data too short".to_string());
    }

    // Get the first 8 hex characters (4 bytes)
    let version_hex = &raw_tx_hex[0..8];

    // Convert hex to bytes
    let bytes = match hex::decode(version_hex) {
        Ok(b) => b,
        Err(_) => return Err("Hex decode error".to_string()),
    };

    // Convert little-endian bytes to u32
    if bytes.len() != 4 {
        return Err("Invalid byte length for version".to_string());
    };

    let version = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    Ok(version)

    // todo!()
}
