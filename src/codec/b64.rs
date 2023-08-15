use super::adapter::Codec;

const UPPERCASEOFFSET: i8 = b'A' as i8; // b'A' is 65 in utf-8, but 0 in Base64. So the offset is b'A'-0.
const LOWERCASEOFFSET: i8 = b'a' as i8 - 26; // b'a' is 97 in utf-8, but represents 26 in Base64. So the offset is b'a'-26=71.
const DIGITOFFSET: i8 = b'0' as i8 - 52; // b'0' is 48 in utf-8, and represents 0 in Base64 (haha). So the offset is b'0'-52=-4
const PADDING: i8 = '=' as i8;

pub struct Base64Adapter;

impl Codec for Base64Adapter {
    fn map_value_to_char(&self, v: u8) -> Option<u8> {
        let v = v as i8;
        let ascii_value = match v {
            0..=25 => v + UPPERCASEOFFSET,
            26..=51 => v + LOWERCASEOFFSET,
            52..=61 => v + DIGITOFFSET,
            62 => 43, // +
            63 => 47, // -

            _ => return None,
            /* We MUST not map any other characters outside the Base64 space.
             * per specification: https://www.rfc-editor.org/rfc/rfc4648#section-3.3
             *
             * > Non-alphabet characters could exist within base-encoded data,
             * > caused by data corruption or by design.  Non-alphabet characters may
             * > be exploited as a "covert channel", where non-protocol data can be
             * > sent for nefarious purposes.  Non-alphabet characters might also be
             * > sent in order to exploit implementation errors leading to, e.g.,
             * > buffer overflow attacks.
             */
        } as u8;

        Some(ascii_value)
    }

    fn map_char_to_value(&self, c: u8) -> Option<u8> {
        //https://base64.guru/learn/base64-characters
        let c = c as i8;
        let base64_index = match c {
            65..=90 => c - UPPERCASEOFFSET,
            97..=127 => c - LOWERCASEOFFSET,
            48..=57 => c - DIGITOFFSET,
            43 => 62, // '+'
            47 => 63, // '/'

            _ => return None, // also ignores PADDING
        } as u8;

        Some(base64_index)
    }

    /* Process 3-byte bundles to encode as Base64.
     */
    fn get_chunksize(&self) -> usize {
        3
    }

    /* Attempt to perform bitwise operations to convert a 3 byte chunk from
    ascii to base64.
    */
    fn raw_encode(&self, chunk: &[u8]) -> Vec<u8> {
        let mut res = match chunk.len() {
            1 => vec![(&chunk[0] & 0b11111100) >> 2, (&chunk[0] & 0b00000011) << 4],
            2 => vec![
                (&chunk[0] & 0b11111100) >> 2,
                (&chunk[0] & 0b00000011) << 4 | (&chunk[1] & 0b11110000) >> 4,
                (&chunk[1] & 0b00001111) << 2,
            ],
            3 => vec![
                (&chunk[0] & 0b11111100) >> 2,
                (&chunk[0] & 0b00000011) << 4 | (&chunk[1] & 0b11110000) >> 4,
                (&chunk[1] & 0b00001111) << 2 | (&chunk[2] & 0b11000000) >> 6,
                &chunk[2] & 0b00111111,
            ],
            _ => unreachable!(),
        } // after performing bitwise operations, map each resulting byte from u8 to base64 characters
        .iter()
        .filter_map(|c| self.map_value_to_char(*c))
        .collect::<Vec<u8>>();

        while res.len() < 4 {
            res.push(PADDING as u8);
        } // Inelegant, but we MUST pad only after mapping values to base64 chars
          // because the padding characters must lie outside the mapped space.

        res
    }

    fn raw_decode(&self, data: &[u8]) -> Vec<u8> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::Base64Adapter;
    use crate::codec::adapter::Codec;

    fn factory() -> Base64Adapter {
        Base64Adapter {}
    }

    #[test]
    fn test_single_char() {
        let input_str = "a";
        let expected = "YQ==";

        let input_data = input_str.as_bytes();

        assert_eq!(factory().encode_to_string(input_data), expected);
    }

    #[test]
    fn test_two_chars() {
        let input_str = "ab";
        let expected = "YWI=";

        let input_data = input_str.as_bytes();

        assert_eq!(factory().encode_to_string(input_data), expected);
    }

    #[test]
    fn test_three_chars() {
        let input_str = "abc";
        let expected = "YWJj";

        let input_data = input_str.as_bytes();

        assert_eq!(factory().encode_to_string(input_data), expected);
    }

    #[test]
    fn tests_short_string() {
        let input_str = "Hello, world!";
        let expected = "SGVsbG8sIHdvcmxkIQ==";

        let input = input_str.as_bytes();

        assert_eq!(factory().encode_to_string(input), expected);
    }

    #[test]
    fn test_longer_string() {
        let input_str = "And here be a bit longer text. Let's see how it goes!";
        let expected = "QW5kIGhlcmUgYmUgYSBiaXQgbG9uZ2VyIHRleHQuIExldCdzIHNlZSBob3cgaXQgZ29lcyE=";

        let input_data = input_str.as_bytes();

        assert_eq!(factory().encode_to_string(input_data), expected);
    }
}
