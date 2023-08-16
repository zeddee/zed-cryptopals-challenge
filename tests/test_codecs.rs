
use cryptochallenge::codec::*;

#[test]
fn test_hex_to_b64() {
    let case = (
        "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d",
        "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t",
    );
    
    let res = hex_to_b64_string(case.0);
    //let expected: Vec<char> = case.1.chars().collect();

    assert_eq!(res, case.1);
}

#[test]
fn test_b64_to_hex() {
    let case = (
        "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t",
        "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d",
    );
    
    let res = b64_to_hex_string(case.0);
    //let expected: Vec<char> = case.1.chars().collect();

    assert_eq!(res, case.1);
}