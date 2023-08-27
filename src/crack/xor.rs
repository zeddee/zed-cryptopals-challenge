use crate::codec::adapter::Codec;
use crate::crack::DecryptResult;
use std::sync::Arc;
use std::thread;

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

/// Perform a brute force attack on ``crypt_text`` using
/// a single byte cipher, operating in the encoding format specified by `codec`.
pub fn brute<T>(codec: &T, crypt_text: &str) -> Vec<u8>
where
    T: Codec + Copy + std::marker::Sync + std::marker::Send + 'static,
{
    let mut leader = DecryptResult {
        // cipher: vec![0],
        score: 0,
        decrypted_result: vec![0],
    };
    let codec: Arc<T> = Arc::new(*codec);
    let brute_cipher_max: u8 = 255;

    let mut queue: Vec<thread::JoinHandle<DecryptResult>> = Vec::new();
    for i in 0..=brute_cipher_max {
        let codec: Arc<T> = Arc::clone(&codec);
        queue.push(async_brute_sub(
            &codec,
            crypt_text.as_bytes().to_vec(),
            vec![i],
        ))
    }

    for q in queue {
        if let Ok(res) = q.join() {
            if leader.score < res.score {
                leader = res
            }
        }
    }

    leader.decrypted_result
}

/// Provide an async helper function for [crate::crack::xor::brute]
/// to run [crate::crack::xor::xor_decrypt]
/// asynchronously.
fn async_brute_sub<'a, 'b, T>(
    codec: &Arc<T>,
    crypt_text: Vec<u8>,
    cipher: Vec<u8>,
) -> thread::JoinHandle<DecryptResult>
where
    T: Codec + std::marker::Sync + std::marker::Send + 'static,
{
    let codec = Arc::clone(codec);
    thread::spawn(move || {
        let cipher_hex = codec.encode(cipher.as_slice());
        let decrypt_res = codec.decode(
            xor_decrypt(codec.as_ref(), crypt_text.as_slice(), cipher_hex.as_slice()).as_slice(),
        );
        let current_ascii_score = DecryptResult {
            score: ascii_score(decrypt_res.clone()),
            //cipher: cipher,
            decrypted_result: decrypt_res,
        };

        current_ascii_score
    })
}

/// Decrypt byte-slice of content with a given key, using repeated key xor.
pub fn xor_decrypt<T: Codec>(codec: &T, encoded_content: &[u8], encoded_key: &[u8]) -> Vec<u8> {
    let mut res = codec.decode(encoded_content);
    let key = codec.decode(encoded_key);
    for k in key {
        res = res.iter().map(|x| x ^ k).collect::<Vec<u8>>();
    }
    codec.encode(&res)
}

/// XOR encrypts ASCII byte-slice `content` with a byte slice `key`.
pub fn xor_encrypt<T: Codec>(codec: &T, content: &[u8], key: &[u8]) -> Vec<u8> {
    let encoded_content = codec.encode(content);

    for mut b in &encoded_content {
        let mut _buf: u8 = 0;
        for k in key {
            _buf = b ^ k;
            b = &_buf;
        }
    }
    encoded_content
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codec::hex::Hexadecimal;

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

    #[test]
    fn test_xor_encrypt() {
        let case = (
            "Cooking MC's like a pound of bacon".as_bytes(),
            "58".as_bytes(),
        );

        let res = xor_encrypt(&factory(), case.0, case.1);

        assert_eq!(
            Hexadecimal {}.encode_to_string(res.as_slice()),
            "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"
        );
    }

    /// Simulate a line break in a text file, as opposed to encoded `\r\n` chars
    #[test]
    fn test_multiline_xor_decrypt() {
        let input = "4275726e696e672027656d2c20696620796f752061696e277420717569636b20616e64206e696d626c650d\n4920676f206372617a79207768656e2049206865617220612063796d62616c".as_bytes();
        let expected = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let codec = factory();
        let res = xor_decrypt(&codec, input, &[1]);

        assert_eq!(
            codec.decode_to_string(res.as_slice()),
            expected,
        )
    }

    /// Simulate a line break in a text file, as opposed to encoded `\r\n` chars
    #[test]
    fn test_multiline_xor_encrypt() {
        let input = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let expected = "4275726e696e672027656d2c20696620796f752061696e277420717569636b20616e64206e696d626c650a4920676f206372617a79207768656e2049206865617220612063796d62616c";
        let codec = factory();
        let res = xor_encrypt(&codec, input.as_bytes(), &[1]);
        println!("as there any result? {:?}", res);
        assert_eq!(
            res.iter().map(|&c| c as char).collect::<String>(),
            expected,
        )
    }
}
