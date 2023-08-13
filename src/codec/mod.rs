pub mod adapter;
pub mod b64;
pub mod hex;

pub fn hex_to_b64_string(data: &str) -> String{
  b64::encode_to_string(&hex::decode_string(data).as_slice())
}
