use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha = Hmac<Sha256>;

pub fn generate_totp(dh_key: &[u8], current_time: u64, time_limit: u64, number_of_digits: u32) -> u32 {
    let count = current_time/time_limit;
    let msg = count.to_be_bytes();

    let mut mac = HmacSha::new_from_slice(dh_key).expect("Invalid key");
    mac.update(&msg);

    let hmac_result = mac.finalize().into_bytes();

    let offset = (hmac_result[hmac_result.len() - 1] & 0x0f) as usize;

    let binary = ((hmac_result[offset] & 0x7f) as u32) << 24
        | (hmac_result[offset + 1] as u32) << 16
        | (hmac_result[offset + 2] as u32) << 8
        | (hmac_result[offset + 3] as u32);

    let modulo = 10u32.pow(number_of_digits);
    binary % modulo
}
    
