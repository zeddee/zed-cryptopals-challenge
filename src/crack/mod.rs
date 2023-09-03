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

/// The Hamming distance between two byte slices is the number of bits that
/// are different in these two byte slices.
pub fn hamming_distance(b1: &[u8], b2: &[u8]) -> usize {
    let table = b1.iter().zip(b2);
    let mut out: usize = 0;
    for (x, y) in table {
        let xor_result = x ^ y;
        out += xor_result.count_ones() as usize;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codec::b64::Base64;

    fn _factory() -> Base64 {
        Base64 {}
    }

    #[test]
    fn hamming_distance_wokka() {
        let cases = [("this is a test", "wokka wokka!!!", 37)];
        for case in cases {
            let h = hamming_distance(case.0.as_bytes(), case.1.as_bytes());
            assert_eq!(h, case.2);
        }
    }
}
