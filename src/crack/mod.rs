//! # Crack
//!
//! This module contains utilities for encrypting and decrypting data.
//! Implemented as Zed works through <https://cryptopals.com/>.

pub mod xor;

/// DecryptResult is used to store
/// a decrypted vector of bytes,
/// and a score for this result instance.
///
/// Intended to be used to compare scores
/// across a series of decrypted results
#[derive(Clone, Debug)]
pub struct DecryptResult {
    key: Vec<u8>,
    decrypted_result: Vec<u8>,
    score: usize,
}

impl DecryptResult {
    pub fn get_decrypted_result(&self) -> Vec<u8> {
        self.decrypted_result.clone()
    }

    pub fn get_key(&self) -> Vec<u8> {
        self.key.clone()
    }
}
