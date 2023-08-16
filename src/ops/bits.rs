use crate::codec::adapter::Codec;
use crate::codec::hex::Hexadecimal;

pub fn xor_two_hexes(hex1: &[u8], hex2: &[u8]) -> Vec<u8> {
    let codec = &Hexadecimal {};
    let d1 = codec.decode(hex1);
    let d2 = codec.decode(hex2);

    let res = d1
        .iter()
        .zip(d2.iter())
        .map(|(l, h)| l ^ h)
        .collect::<Vec<u8>>();

    codec.encode(res.as_slice())
}

pub fn xor_decrypt_hex(crypt_text: &[u8], cipher: &[u8]) -> Vec<u8> {
    crypt_text
        .chunks(cipher.len())
        .flat_map(|x| xor_two_hexes(x, &cipher))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xor_two_hexes() {
        let case = (
            "1c0111001f010100061a024b53535009181c".as_bytes(),
            "686974207468652062756c6c277320657965".as_bytes(),
            "746865206b696420646f6e277420706c6179".as_bytes(),
        );

        let res = xor_two_hexes(case.0, case.1);

        assert_eq!(res, case.2);
    }

    #[test]
    fn test_xor_decrypt_hex() {
        let case = (
            "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".as_bytes(),
            "58".as_bytes(),
        );

        let res = xor_decrypt_hex(case.0, case.1);

        assert_eq!(
            Hexadecimal {}.decode_to_string(res.as_slice()),
            "Cooking MC's like a pound of bacon"
        );
    }
}
