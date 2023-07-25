fn test(buff: &mut [u8]) -> Result<&mut [u8], &str> {
    let data = &mut buff[3..25];
    Ok(data)
}

fn main() {
    let mut buf = [1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 4, 8, 9, 10, 11, 12, 13, 14, 15, 16, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,]; // From PackedSize
    {
        println!("{:?}", &buf);
        if let Ok(data) = test(&mut buf) {
            println!(" {:?}", &data);
        }
    }
}
