/*
EncodingAdapter trait describes an interface for types
that implement an API for different representations of integer values.

E.g. Base64, Hexadecimal
*/
pub trait EncodingAdapter {
    fn map_value_to_char(&self, v: u8) -> Option<u8>;
    fn map_char_to_value(&self, c: u8) -> Option<u8>;
    fn raw_encode(&self, v: &[u8]) -> Vec<u8>;
    fn encode_raw_chunk(&self, chunk: &[u8]) -> Vec<u8>;

    fn encode(&self, data: &[u8]) -> Vec<u8> {
        data.chunks(3)
            .flat_map(|c| self.encode_raw_chunk(c))
            .collect::<Vec<u8>>()
    }
    fn encode_to_string(&self, data: &[u8]) -> String {
        self.encode(data)
            .iter()
            .map(|v| (*v as char).to_string())
            .collect::<String>()
    }
}
