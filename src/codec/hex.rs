use super::adapter::EncodingAdapter;

struct Hexadecimal;
const UPPERCASEOFFSET: u8 = b'A'; // 65 in utf8, but 10 in hex (hex is case insensitive)
const LOWERCASEOFFSET: u8 = b'a'; // 97 in utf8, but 10 in hex
const DIGITOFFSET: u8 = b'0'; // 48

impl EncodingAdapter for Hexadecimal {
    fn value_to_char(&self, v: u8) -> Option<char> {
        match v {
            0..=9 => Some((v + DIGITOFFSET) as char),
            10..=35 => Some((v - 10 + LOWERCASEOFFSET) as char), // Hex is case insensitive
            _ => None,
        }
    }
    fn char_to_value(&self, c: char) -> Option<u8> {
        let b = c as u8;
        match b {
            b'0'..=b'9' => Some(b - DIGITOFFSET),
            b'a'..=b'z' => Some(b - LOWERCASEOFFSET + 10), // Hex is case insensitive
            b'A'..=b'Z' => Some(b - UPPERCASEOFFSET + 10),
            _ => None,
        }
    }
}

/*
Takes a string of hexadecimal values and returns a vector of 8-bit/4 byte values.

hexadecimal strings are a series of 4 bit values
represented in an 8-bit space.
So when we decode hex to bytes, we want to remove the 4 bits of padding
and concatenate each of the actual important 4 bits into an 8-bit space.
*/
pub fn decode(data: &[u8]) -> Vec<u8> {
    let encoder = &Hexadecimal {};
    let mut raw = data
        .iter()
        .filter_map(|c| encoder.char_to_value(*c as char)); // Use filter_map because we're returning an Option<_>

    let mut bytes: Vec<u8> = Vec::new();
    while let (Some(h), Some(l)) = (raw.next(), raw.next()) {
        bytes.push(h << 4 | l)
    }
    bytes
}

/* Convenience wrapper over hex::decode(data: &[u8])
 */
pub fn decode_string(data: &str) -> Vec<u8> {
    let bytes = data.as_bytes();
    decode(bytes)
}
