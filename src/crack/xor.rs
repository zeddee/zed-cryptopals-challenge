use crate::{codec::adapter::Codec, crack::DecryptResult};
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
        let decrypt_res = codec.to_plain(
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
    let key = codec.to_plain(key);
    let keyslice = key.as_slice();

    // Decrypt step 1: Split at newlines in content, make iterable to operate on
    let content_string = content.iter().map(|c| *c as char).collect::<String>();
    let content_string_lines = content_string.lines();

    // Decrypt step 2: Run XOR at byte level for each line
    let mut outer_res: Vec<Vec<u8>> = Vec::new();
    for line in content_string_lines {
        // Use `codec` to decode this line
        let decoded_line = codec.to_plain(line.as_bytes());

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
                joined.push(b'\\');
                joined.push(b'n');
            }
        }
    }
    joined
}

/// XOR encrypts ASCII byte-slice `content`
/// with an encoded byte slice `key`.
pub fn xor_encrypt<T: Codec>(codec: &T, content: &[u8], key: &[u8]) -> Vec<u8> {
    // Convert content to string and then split it into lines
    let content = content.iter().map(|&c| c as char).collect::<String>();
    let content_lines = content.lines();

    // encode, because we need to iterate over the correct chunk size
    // assume `key` is already encoded
    let encoded_content_lines = content_lines.map(|line| codec.encode(line.as_bytes()));

    let mut encrypted_lines: Vec<Vec<u8>> = Vec::new();
    // Must process in chunk sizes that match the byte-slice size of the key.
    for encoded in encoded_content_lines {
        let this_line = encoded
            .chunks(key.len())
            .flat_map(|chunk| {
                let decoded_chunk = codec.to_plain(chunk);
                let inner_key = codec.to_plain(key);
                decoded_chunk
                    .iter()
                    .zip(inner_key)
                    .map(|(l, h)| l ^ h)
                    .collect::<Vec<u8>>()
            })
            .collect::<Vec<u8>>();

        encrypted_lines.push(this_line);
    }

    let mut joined: Vec<u8> = Vec::new();
    let mut it = encrypted_lines.iter().peekable();
    while it.peek().is_some() {
        while let Some(inner_res) = it.next() {
            for res in inner_res {
                joined.push(*res);
            }
        }
        if it.peek().is_some() {
            joined.push(b'0');
            joined.push(b'a');
        }
    }

    joined
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codec::adapter::CodecAPI;
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
            res.get_decrypted_result()
                .iter()
                .map(|c| *c as char)
                .collect::<String>(),
            "Cooking MC's like a pound of bacon"
        );
    }

    #[test]
    fn test_xor_decrypt() {
        let cases = [
            (
                "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".as_bytes(),
                "58".as_bytes(),
            ),
            (
                "1b38373c31393f7715147f24783b313c3d77397728382d393c773731783539343739".as_bytes(),
                "5857".as_bytes(),
            ),
        ];

        for case in cases {
            let res = xor_decrypt(&factory(), case.0, case.1);

            assert_eq!(
                Hexadecimal {}.to_plain_string(res.as_slice()),
                "Cooking MC's like a pound of bacon"
            );
        }
    }

    #[test]
    fn test_xor_encrypt() {
        let cases = [
            (
                "Cooking MC's like a pound of bacon".as_bytes(),
                "58".as_bytes(),
                "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736",
            ),
            (
                "Cooking MC's like a pound of bacon".as_bytes(),
                "5857".as_bytes(),
                "1b38373c31393f7715147f24783b313c3d77397728382d393c773731783539343739",
            ),
        ];

        for case in cases {
            let res = xor_encrypt(&factory(), case.0, case.1);

            assert_eq!(&factory().encode_to_string(res.as_slice()), case.2,);
        }
    }

    /// Simulate a line break in a text file, as opposed to encoded `\r\n` chars
    #[test]
    fn test_multiline_xor_decrypt() {
        let input = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272
a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"
            .as_bytes();
        let expected = "Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal";
        let key = "58".as_bytes();
        let codec = factory();
        let res = xor_decrypt(&codec, input, key);

        assert_eq!(codec.to_plain_string(res.as_slice()), expected,)
    }

    /// Simulate a line break in a text file, as opposed to encoded `\r\n` chars
    #[test]
    fn test_multiline_xor_encrypt() {
        let input = "Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal";
        let expected = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272
a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
        let key = "58".as_bytes();
        let codec = factory();
        let res = xor_encrypt(&codec, input.as_bytes(), key);
        assert_eq!(codec.encode_to_string(res.as_slice()), expected,)
    }

    const REPEATEDXOR_UNENCRYPTED: &str = "Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal";
    const REPEATEDXOR_ENCRYPTED: &str =
        "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272
a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";

    #[test]
    fn test_encrypt_repeated_xor() {
        let codec = factory();
        let key = codec.encode("ICE".as_bytes());

        assert_eq!(
            codec.encode_to_string(
                xor_encrypt(&codec, REPEATEDXOR_UNENCRYPTED.as_bytes(), key.as_slice()).as_slice()
            ),
            REPEATEDXOR_ENCRYPTED,
        );
    }

    #[test]
    fn test_decrypt_repeated_xor() {
        let codec = factory();
        let key = codec.encode("ICE".as_bytes());
        let res = codec.to_plain_string(
            xor_decrypt(&codec, REPEATEDXOR_ENCRYPTED.as_bytes(), key.as_slice()).as_slice(),
        );
        assert_eq!(res, REPEATEDXOR_UNENCRYPTED,);
    }
}
