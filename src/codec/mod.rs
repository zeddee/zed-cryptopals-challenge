pub mod adapter;
pub mod b64;
pub mod hex;

use super::codec::adapter::Codec;

pub fn hex_to_b64_string(data: &str) -> String {
    let b64_codec = &b64::Base64Adapter {};
    b64_codec.encode_to_string(&hex::decode_string(data).as_slice())
}
