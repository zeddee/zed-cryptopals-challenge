//! # Crack
//!
//! This module contains utilities for encrypting and decrypting data.
//! Implemented as Zed works through <https://cryptopals.com/>.

pub mod xor;
pub mod xor_repeating_key;

/// DecryptResult is used to store
/// a decrypted vector of bytes,
/// and a score for this result instance.
///
/// Intended to be used to compare scores
/// across a series of decrypted results
#[derive(Debug)]
struct DecryptResult {
    // cipher: Vec<u8>,
    decrypted_result: Vec<u8>,
    score: usize,
}
