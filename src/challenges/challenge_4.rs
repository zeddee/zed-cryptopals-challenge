//! Set 1/Challenge 4 <https://cryptopals.com/sets/1/challenges/4>

use crate::codec::hex::Hexadecimal;
use crate::crack::xor;

/// One-off function to:
/// 1. Read a file containing one hex per line.
/// 1. Decrypt each line with xor_decrypt::brute.
/// 1. Score the result with xor_decrypt::decrypt_score.
/// 1. Return the decrypted string that scores the highest.
pub fn find_encrypted_string(filename: &str) -> String {
    let mut res: (String, usize) = (String::from(""), 0);
    let codec = &Hexadecimal {};
    let crypt_list = crate::utils::fs::read_file(filename);

    for crypt_line in crypt_list {
        let line_decrypt = xor::brute(codec, &crypt_line.as_str());
        let score = xor::ascii_score(line_decrypt.clone());
        let line_decrypt_string = line_decrypt.iter().map(|c| *c as char).collect::<String>();
        println!("decrypted: {}", line_decrypt_string);
        if score > res.1 {
            res = (line_decrypt_string, score);
        }
    }
    res.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_encrypted_string() {
        let res = find_encrypted_string("_data/set1/challenge4.txt");
        let expected = "Now that the party is jumping\n";
        assert_eq!(res, expected);
    }
}
