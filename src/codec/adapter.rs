/*
EncodingAdapter trait describes an interface for types
that implement an API for different representations of integer values.

E.g. Base64, Hexadecimal
*/
pub trait Codec {
    fn map_value_to_char(&self, v: u8) -> Option<u8>;
    fn map_char_to_value(&self, c: u8) -> Option<u8>;

    /* Takes a slice of octets
    as input, coerces the bits of those octets
    to fit the bit width (bits required to store base-encoded data)
    of the target encoding.
     */
    fn raw_encode(&self, v: &[u8]) -> Vec<u8>;

    /* Return number of bytes we need per processed chunk.
    E.g. Base64 must process bundles of 3 bytes.
    Using function to return single value because `const` declarations
    not supported in stable.
     */
    fn get_chunksize(&self) -> usize {
        4
    }

    fn encode(&self, data: &[u8]) -> Vec<u8> {
        data.chunks(self.get_chunksize())
            .flat_map(|c| self.raw_encode(c))
            .collect::<Vec<u8>>()
    }
    fn encode_to_string(&self, data: &[u8]) -> String {
        self.encode(data)
            .iter()
            .map(|v| (*v as char).to_string())
            .collect::<String>()
    }

    fn raw_decode(&self, v: &[u8]) -> Vec<u8>;

    fn decode(&self, data: &[u8]) -> Vec<u8> {
        data.chunks(4)
            .flat_map(|c| self.raw_decode(c))
            .collect::<Vec<u8>>()
    }
    fn decode_to_string(&self, data: &[u8]) -> String {
        self.decode(data)
            .iter()
            .map(|v| (*v as char).to_string())
            .collect::<String>()
    }
}
