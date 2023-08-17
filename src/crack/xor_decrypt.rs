use crate::codec::adapter::Codec;
use crate::codec::hex::Hexadecimal;
use crate::ops::bits::*;

#[derive(Debug)]
struct DecryptResult {
    // cipher: Vec<u8>,
    score: usize,
    decrypted_result: Vec<u8>,
}

// Read a series of characters and assign a score for each.
// The higher the score, the more ASCII characters are in the input data,
// and the more likely it is human-readable text.
fn decrypt_score(score_me: Vec<u8>) -> usize {
    score_me
        .iter()
        .map(|&c| {
            let res_c = match c {
                65..=90 => 10,  // ASCII Uppercase alphabet
                97..=122 => 10, // ASCII Lowercase alphabet
                48..=57 => 5,   // ASCII Digit
                32 => 5,        // Space
                33..=47 => 1,   // ASCII punctuation
                58..=64 => 1,   // More ASCII punctuation
                91..=96 => 1,   // More ASCII punctuation
                123..=126 => 1, // More ASCII punctuation
                _ => 0,         // disregard non-legible characters
            };
            res_c
        })
        .fold(0, |x, acc| x + acc as usize)
}

pub fn brute(crypt_text: &str) -> Vec<u8> {
    let mut leader = DecryptResult {
        // cipher: vec![0],
        score: 0,
        decrypted_result: vec![0],
    };
    let mut brute_cipher: u8 = 0;

    // assume cipher is any one byte character
    // brute force by trying all one byte characters.
    while brute_cipher < 255 {
        let cipher = Hexadecimal {}.encode(&[brute_cipher]);

        let decrypt_res = Hexadecimal {}
            .decode(xor_decrypt_hex(crypt_text.as_bytes(), cipher.as_slice()).as_slice());

        let current_decrypt_score = DecryptResult {
            score: decrypt_score(decrypt_res.clone()),
            //cipher: cipher,
            decrypted_result: decrypt_res,
        };

        if leader.score < current_decrypt_score.score {
            leader = current_decrypt_score
        }

        brute_cipher += 1;
    }

    leader.decrypted_result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xor_decrypt_brute() {
        let input =
            "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".as_bytes();
        let expected_cipher = "58".as_bytes();

        println!("Expected cipher: {:?}", expected_cipher);

        let res = brute(
            input
                .iter()
                .map(|c| *c as char)
                .collect::<String>()
                .as_str(),
        );

        assert_eq!(
            res.iter().map(|c| *c as char).collect::<String>(),
            "Cooking MC's like a pound of bacon"
        );
    }
}
