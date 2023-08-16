pub mod adapter;
pub mod b64;
pub mod hex;

use super::codec::adapter::Codec;

pub fn hex_to_b64_string(data: &str) -> String {
    b64::Base64Adapter {}.encode_to_string(
        &hex::Hexadecimal {}.decode(data.as_bytes()
    ).as_slice())
}

pub fn b64_to_hex_string(data: &str) -> String {
    hex::Hexadecimal {}.encode_to_string(
        &b64::Base64Adapter {}.decode(data.as_bytes()
    ).as_slice())
}
