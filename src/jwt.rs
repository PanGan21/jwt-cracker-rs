use jsonwebtoken::{decode_header, Algorithm};
use serde_json::Value;

pub fn decode_jwt(token: &str) -> Result<(jsonwebtoken::Header, Value, String, Vec<u8>), String> {
    let header = decode_header(token).map_err(|e| format!("Failed to decode JWT header: {}", e))?;

    if !matches!(
        header.alg,
        Algorithm::HS256 | Algorithm::HS384 | Algorithm::HS512
    ) {
        return Err(format!("Unsupported algorithm: {:?}", header.alg));
    }

    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return Err(
            "Invalid JWT token. Ensure it contains a header, payload, and signature.".into(),
        );
    }

    let signature = base64::decode_config(parts[2], base64::URL_SAFE_NO_PAD)
        .map_err(|e| format!("Failed to decode signature: {}", e))?;
    let message = format!("{}.{}", parts[0], parts[1]);

    let payload_bytes =
        base64::decode_config(parts[1], base64::URL_SAFE_NO_PAD).map_err(|e| format!("{}", e))?;
    let payload: Value = serde_json::from_slice(&payload_bytes)
        .map_err(|e| format!("Failed to parse JWT payload: {}", e))?;

    Ok((header, payload, message, signature))
}
