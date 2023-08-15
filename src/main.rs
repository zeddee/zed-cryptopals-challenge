use cryptochallenge::codec::adapter::Codec;
use cryptochallenge::codec::{b64, hex};

fn main() {
    let hex_bytes = &hex::Hexadecimal{}.decode(
        "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"
            .as_bytes()
    );
    println!(
        "{}",
        hex_bytes
            .iter()
            .map(|c| (*c as char).to_string())
            .collect::<String>()
    );

    let b64_string = b64::Base64Adapter{}.encode_to_string(hex_bytes.as_slice());
    println!("{:?}", b64_string,);
}
