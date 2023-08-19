use crate::codec::adapter::Codec;
use crate::codec::hex::Hexadecimal;
use crate::crack::DecryptResult;

// Read a series of characters and assign a score for each.
// The higher the score, the more ASCII characters are in the input data,
// and the more likely it is human-readable text.
pub fn ascii_score(score_me: Vec<u8>) -> usize {
    score_me
        .iter()
        .map(|&c| {
            let res_c = match c {
                65..=90 => 20,  // ASCII Uppercase alphabet
                97..=122 => 20, // ASCII Lowercase alphabet
                48..=57 => 10,  // ASCII Digit
                32 => 20,       // Space
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

pub fn brute<T: Codec>(codec: &T, crypt_text: &str) -> Vec<u8> {
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
            .decode(xor_decrypt(codec, crypt_text.as_bytes(), cipher.as_slice()).as_slice());

        let current_ascii_score = DecryptResult {
            score: ascii_score(decrypt_res.clone()),
            //cipher: cipher,
            decrypted_result: decrypt_res,
        };

        if leader.score < current_ascii_score.score {
            leader = current_ascii_score
        }

        brute_cipher += 1;
    }

    leader.decrypted_result
}

/* Limitation -- we zip the two byte slices,
which truncates both slices to the length of the shorter slice in the resulting (&[u8], &[u8]) tuple.
 */
pub fn xor_two<T: Codec>(codec: &T, hex1: &[u8], hex2: &[u8]) -> Vec<u8> {
    let d1 = codec.decode(hex1);
    let d2 = codec.decode(hex2);

    let res = d1
        .iter()
        .zip(d2.iter())
        .map(|(l, h)| l ^ h)
        .collect::<Vec<u8>>();

    codec.encode(res.as_slice())
}

pub fn xor_decrypt<T: Codec>(codec: &T, crypt_text: &[u8], cipher: &[u8]) -> Vec<u8> {
    crypt_text
        .chunks(cipher.len())
        .flat_map(|x| xor_two(codec, x, &cipher))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn factory() -> Hexadecimal {
        Hexadecimal {}
    }

    #[test]
    fn test_xor_decrypt_brute() {
        let input =
            "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".as_bytes();
        let expected_cipher = "58".as_bytes();

        println!("Expected cipher: {:?}", expected_cipher);

        let res = brute(
            &factory(),
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

    #[test]
    fn test_xor_two_hexes() {
        let case = (
            "1c0111001f010100061a024b53535009181c".as_bytes(),
            "686974207468652062756c6c277320657965".as_bytes(),
            "746865206b696420646f6e277420706c6179".as_bytes(),
        );

        let res = xor_two(&factory(), case.0, case.1);

        assert_eq!(res, case.2);
    }

    #[test]
    fn test_xor_decrypt() {
        let case = (
            "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".as_bytes(),
            "58".as_bytes(),
        );

        let res = xor_decrypt(&factory(), case.0, case.1);

        assert_eq!(
            Hexadecimal {}.decode_to_string(res.as_slice()),
            "Cooking MC's like a pound of bacon"
        );
    }
}
