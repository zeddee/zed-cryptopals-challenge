use super::adapter::EncodingAdapter;

struct Base64;
const UPPERCASEOFFSET: i8 = b'A' as i8; // b'A' is 65 in utf-8, but 0 in Base64. So the offset is b'A'-0.
const LOWERCASEOFFSET: i8 = b'a' as i8 - 26; // b'a' is 97 in utf-8, but represents 26 in Base64. So the offset is b'a'-26=71.
const DIGITOFFSET: i8 = b'0' as i8 - 52; // b'0' is 48 in utf-8, and represents 0 in Base64 (haha). So the offset is b'0'-52=-4
const PADDING: char = '=';

impl EncodingAdapter for Base64 {
    fn value_to_char(&self, v: u8) -> Option<char> {
        let v = v as i8;
        let ascii_value = match v {
            0..=25 => v + UPPERCASEOFFSET,
            26..=51 => v + LOWERCASEOFFSET,
            52..=61 => v + DIGITOFFSET,
            62 => 43, // +
            63 => 47, // -

            _ => return None,
        } as u8;

        Some(ascii_value as char)
    }
    fn char_to_value(&self, c: char) -> Option<u8> {
        //https://base64.guru/learn/base64-characters
        let c = c as i8;
        let base64_index = match c {
            65..=90 => c - UPPERCASEOFFSET,
            97..=127 => c - LOWERCASEOFFSET,
            48..=57 => c - DIGITOFFSET,
            43 => 62, // '+'
            47 => 63, // '/'

            _ => return None,
        } as u8;

        Some(base64_index)
    }
}

/* Attempt to perform bitwise operations to convert a 3 byte chunk from
ascii to base64.
*/
fn split_bits(chunk: &[u8]) -> Vec<u8> {
    match chunk.len() {
      1 => vec![ // If chunk size is 1 byte, return 2 bytes
        &chunk[0] >> 2, // shifts bits of the first byte 2 bits to the right. Effectively truncates the last two bits of the byte.
        (&chunk[0] & 0b00000011) << 4, // selects the last 2 bits, and shifts them 4 bits left.
      ],
      2 => vec![
        &chunk[0] >> 2,
        (&chunk[0] & 0b00000011) << 4 | &chunk[1] >> 4, // Set the second byte by performing an inclusive OR between the first and second bytes
        (&chunk[1] & 0b00001111) << 2, // For the second byte, shift the last 4 bits 2 bits to the left
      ],
      3 => vec![
        &chunk[0] >> 2,
        (&chunk[0] & 0b00000011) << 4 | &chunk[1] >> 4, // Set the second byte by performing an inclusive OR between the first and second bytes
        (&chunk[1] & 0b00001111) << 2 | &chunk[2] >> 6,
        &chunk[2] & 0b00111111, // select only the first 6 bits of the 3rd byte
      ],
      _ => unreachable!(),
    }
}

fn encode_chunk<T: EncodingAdapter>(alphabet: &T, chunk: Vec<u8>) -> Vec<char> {
  let mut out = vec![PADDING; 4];

  for i in 0..chunk.len() {
    if let Some(c) = alphabet.value_to_char(chunk[i]) {
      //println!("Replacing {:?} with {:?}", out[i], c as char);
      out[i] = c as char;
    }
  }
  out
}

pub fn encode_to_string(data: &[u8]) -> String {
  //let classic_alphabet = &Bridge {};
  encode(data).iter().map(|v| v.to_string()).collect::<String>()
}

pub fn encode(data: &[u8]) -> Vec<char> {
    let bridge = &Base64 {};
    let chunked = data.chunks(3);

    let mut bitshifted = Vec::new();
    for c in chunked {
      bitshifted.push(split_bits(c));
    };

    let mut out = Vec::new();
    for b in bitshifted {
      for c in encode_chunk(bridge, b) {
        out.push(c)
      };
    };

    out
}

#[cfg(test)]
mod tests {
    use super::encode_to_string;

    #[test]
    fn test_single_char() {
        let input_str = "a";
        let expected = "YQ==";

        let input_data = input_str.as_bytes();

        assert_eq!(encode_to_string(input_data), expected);
    }

    #[test]
    fn test_two_chars() {
        let input_str = "ab";
        let expected = "YWI=";

        let input_data = input_str.as_bytes();

        assert_eq!(encode_to_string(input_data), expected);
    }

    #[test]
    fn test_three_chars() {
        let input_str = "abc";
        let expected = "YWJj";

        let input_data = input_str.as_bytes();

        assert_eq!(encode_to_string(input_data), expected);
    }

    #[test]
    fn tests_short_string() {
        let input_str = "Hello, world!";
        let expected = "SGVsbG8sIHdvcmxkIQ==";

        let input = input_str.as_bytes();

        assert_eq!(encode_to_string(input), expected);
    }

    #[test]
    fn test_longer_string() {
        let input_str = "And here be a bit longer text. Let's see how it goes!";
        let expected = "QW5kIGhlcmUgYmUgYSBiaXQgbG9uZ2VyIHRleHQuIExldCdzIHNlZSBob3cgaXQgZ29lcyE=";

        let input_data = input_str.as_bytes();

        assert_eq!(encode_to_string(input_data), expected);
    }
}