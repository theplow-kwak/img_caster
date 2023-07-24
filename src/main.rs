use endian_codec::{PackedSize, EncodeLE, DecodeLE, EncodeBE, DecodeBE};
// If you look at this structure without checking the documentation, you know it works with
// little-endian notation
#[derive(Debug, PartialEq, Eq, PackedSize, EncodeLE, DecodeLE)]
struct Version {
  major: u32,
  minor: u32,
  patch: u32
}

#[derive(Debug, PartialEq, Eq, PackedSize, EncodeBE, DecodeBE)]
struct Version_be {
  major: u32,
  minor: u32,
  patch: u32
}


fn main() {
    let mut buf = [0; Version::PACKED_LEN]; // From PackedSize
    let test = Version { major: 0, minor: 21, patch: 37 };
    println!("org: {:?}", test);
    
    // if you work with big- and little-endians, you will not mix them accidentally
    test.encode_as_le_bytes(&mut buf);
    println!("le: {:?}", buf);

    let test_from_b = Version::decode_from_le_bytes(&buf);
    println!("test_from_b: {:?}", test_from_b);
    
    let mut buf = [0; Version_be::PACKED_LEN]; // From PackedSize
    let test = Version_be { major: 0, minor: 21, patch: 37 };
    println!("org: {:?}", test);
    
    // if you work with big- and little-endians, you will not mix them accidentally
    test.encode_as_be_bytes(&mut buf);
    println!("le: {:?}", buf);

    let test_from_b = Version_be::decode_from_be_bytes(&buf);
    println!("test_from_b: {:?}", test_from_b);
}
