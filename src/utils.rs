use hmac::{Hmac, Mac};
use num_bigint::BigUint;
use sha2::{Sha256, Sha384, Sha512};

type HmacSha256 = Hmac<Sha256>;
type HmacSha384 = Hmac<Sha384>;
type HmacSha512 = Hmac<Sha512>;

pub fn verify_signature(
    message: &[u8],
    signature: &[u8],
    secret: &str,
    alg: &jsonwebtoken::Algorithm,
) -> bool {
    let key = secret.as_bytes();
    match alg {
        jsonwebtoken::Algorithm::HS256 => {
            let mut mac = HmacSha256::new_from_slice(key).unwrap();
            mac.update(message);
            mac.verify_slice(signature).is_ok()
        }
        jsonwebtoken::Algorithm::HS384 => {
            let mut mac = HmacSha384::new_from_slice(key).unwrap();
            mac.update(message);
            mac.verify_slice(signature).is_ok()
        }
        jsonwebtoken::Algorithm::HS512 => {
            let mut mac = HmacSha512::new_from_slice(key).unwrap();
            mac.update(message);
            mac.verify_slice(signature).is_ok()
        }
        _ => false,
    }
}

pub fn increment(current: &mut [usize], base: usize) -> bool {
    for i in (0..current.len()).rev() {
        if current[i] + 1 < base {
            current[i] += 1;
            return true;
        } else {
            current[i] = 0;
        }
    }
    false
}

pub fn calculate_combinations(alphabet: &[u8], max_length: usize) -> BigUint {
    let mut combinations = BigUint::from(0u32);
    let alphabet_len = BigUint::from(alphabet.len() as u32);
    for i in 1..=max_length {
        combinations += alphabet_len.pow(i as u32);
    }
    combinations
}
