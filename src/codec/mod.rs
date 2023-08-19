//! # Codec
//! Codecs for encoding and decoding data.
//!
//! Each codec is an adapter that can be passed to
//! methods that operate on byte slices to convert
//! content from one encoding format to another.
//!
//! A [crate::codec::Codec] implementation describes how bits from an incoming byte
//! slice should be manipulated to produce an encoded or decoded result.

pub mod adapter;
pub mod b64;
pub mod hex;

use crate::codec::adapter::Codec;

/// Convenience function that wraps the `encode_to_string` method of [crate::codec::b64::Base64Adapter]
pub fn hex_to_b64_string(data: &str) -> String {
    b64::Base64Adapter {}.encode_to_string(&hex::Hexadecimal {}.decode(data.as_bytes()).as_slice())
}

/// Convenience function that wraps the `encode_to_string` method of [crate::codec::hex::Hexadecimal].
pub fn b64_to_hex_string(data: &str) -> String {
    hex::Hexadecimal {}.encode_to_string(&b64::Base64Adapter {}.decode(data.as_bytes()).as_slice())
}
