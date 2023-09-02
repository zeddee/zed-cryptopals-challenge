//! # Codec
//! Codecs for encoding and decoding data.
//!
//! Each codec is an adapter that can be passed to
//! methods that operate on byte slices to convert
//! content from one encoding format to another.
//!
//! A [crate::codec::adapter::Codec] implementation describes how bits from an incoming byte
//! slice should be manipulated to produce an encoded or decoded result.

pub mod adapter;
pub mod b64;
pub mod hex;

use crate::codec::adapter::Codec;

/// Convenience function that wraps the `encode_to_string` method of [crate::codec::b64::Base64Adapter]
pub fn hex_to_b64_string(data: &str) -> String {
    b64::Base64Adapter {}.encode_to_string(&hex::Hexadecimal {}.to_utf8(data.as_bytes()).as_slice())
}

/// Convenience function that wraps the `encode_to_string` method of [crate::codec::hex::Hexadecimal].
pub fn b64_to_hex_string(data: &str) -> String {
    hex::Hexadecimal {}.encode_to_string(&b64::Base64Adapter {}.to_utf8(data.as_bytes()).as_slice())
}

#[cfg(test)]

mod tests {

    use super::*;
    #[test]
    fn test_hex_to_b64() {
        let case = (
        "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d",
        "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t",
    );

        let res = hex_to_b64_string(case.0);
        //let expected: Vec<char> = case.1.chars().collect();

        assert_eq!(res, case.1);
    }

    #[test]
    fn test_b64_to_hex() {
        let case = (
        "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t",
        "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d",
    );

        let res = b64_to_hex_string(case.0);
        //let expected: Vec<char> = case.1.chars().collect();

        assert_eq!(res, case.1);
    }
}
