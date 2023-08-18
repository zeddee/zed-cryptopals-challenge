pub mod xor_decrypt;
pub mod xor_repeating_key;

#[derive(Debug)]
struct DecryptResult {
    // cipher: Vec<u8>,
    score: usize,
    decrypted_result: Vec<u8>,
}
