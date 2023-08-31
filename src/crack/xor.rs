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
pub fn brute<T>(codec: &T, crypt_text: &str) -> DecryptResult
where
    T: Codec + Copy + std::marker::Sync + std::marker::Send + 'static,
{
    let mut leader = DecryptResult {
        key: vec![0],
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

    leader
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
            key: cipher_hex,
            score: ascii_score(decrypt_res.clone()),
            decrypted_result: decrypt_res,
        };

        current_ascii_score
    })
}

/// Decrypt byte-slice of content with a given key, using repeated key xor.
/// Returns an encoded vector of bytes.
pub fn xor_decrypt<T: Codec>(codec: &T, content: &[u8], key: &[u8]) -> Vec<u8> {
    let key = codec.decode(key);
    let keyslice = key.as_slice();

    // Decrypt step 1: Split at newlines in content, make iterable to operate on
    let content_string = content.iter().map(|c| *c as char).collect::<String>();
    let content_string_lines = content_string.lines();

    // Decrypt step 2: Run XOR at byte level for each line
    let mut outer_res: Vec<Vec<u8>> = Vec::new();
    for line in content_string_lines {
        // Use `codec` to decode this line
        let decoded_line = codec.decode(line.as_bytes());

        let res = decoded_line
            .chunks(keyslice.len())
            .flat_map(|chunk| {
                chunk
                    .iter()
                    .zip(keyslice)
                    .map(|(l, h)| l ^ h)
                    .collect::<Vec<u8>>()
            })
            .collect::<Vec<u8>>();
        // Push result to `outer_res`.
        outer_res.push(codec.encode(&res))
    }

    // Create buffer for joined output
    let mut joined: Vec<u8> = Vec::new();

    // Convoluted logic for adding newlines `\n` if
    // that line is not the last 'line' of the resulting decrypted text.
    let mut it = outer_res.iter().peekable();
    while it.peek().is_some() {
        if let Some(inner_res) = it.next() {
            for res in inner_res {
                joined.push(*res);
            }
            if it.peek().is_some() {
                joined.push(b'0');
                joined.push(b'A');
            }
        }
    }
    joined
}

/// XOR encrypts ASCII byte-slice `content`
/// with an encoded byte slice `key`.
pub fn xor_encrypt<T: Codec>(codec: &T, content: &[u8], key: &[u8]) -> Vec<u8> {
    // encode, because we need to iterate over the correct chunk size
    // assume `key` is already encoded
    let encoded = codec.encode(content);

    // Must process in chunk sizes that match the byte-slice size of the key.
    encoded
        .chunks(key.len())
        .flat_map(|chunk| {
            let decoded_chunk = codec.decode(chunk);
            let inner_key = codec.decode(key);
            decoded_chunk
                .iter()
                .zip(inner_key)
                .map(|(l, h)| l ^ h)
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<u8>>()
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
            res.decrypted_result.iter().map(|c| *c as char).collect::<String>(),
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
            &factory().encode_to_string(res.as_slice()),
            "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"
        );
    }

    /// Simulate a line break in a text file, as opposed to encoded `\r\n` chars
    #[test]
    fn test_multiline_xor_decrypt() {
        let input = "1a2d2a3631363f787f3d357478313e7821372d783931367f2c78292d313b337839363c783631353a343d5211783f37783b2a392221782f303d36781178303d392a7839783b21353a3934".as_bytes();
        let expected =
            "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let key = "58".as_bytes();
        let codec = factory();
        let res = xor_decrypt(&codec, input, key);

        assert_eq!(codec.decode_to_string(res.as_slice()), expected,)
    }

    /// Simulate a line break in a text file, as opposed to encoded `\r\n` chars
    #[test]
    fn test_multiline_xor_encrypt() {
        let input = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let expected = "1a2d2a3631363f787f3d357478313e7821372d783931367f2c78292d313b337839363c783631353a343d5211783f37783b2a392221782f303d36781178303d392a7839783b21353a3934";
        let key = "58".as_bytes();
        let codec = factory();
        let res = xor_encrypt(&codec, input.as_bytes(), key);
        assert_eq!(codec.encode_to_string(res.as_slice()), expected,)
    }
}
