use std::collections::HashMap;

#[derive(Debug)]
struct Slice {
    slice_no: u32,
    start: u32,
    end: u32,
    slice: u32,
}

impl Slice {
    pub fn new(slice_no: u32) -> Self {
        Self {
            slice_no,
            start: 0,
            end: 0,
            slice: 0,
        }
    }
}

#[derive(Debug)]
struct Man {
    slices: HashMap<u32, Slice>,
    sliceno: u32,
}

impl Man {
    pub fn new() -> Self {
        Self {
            slices: HashMap::new(),
            sliceno: 0,
        }
    }

    pub fn get_slice(&mut self, slice_no: u32) -> &mut Slice {
        if !self.slices.contains_key(&slice_no) {
            let slice = Slice::new(slice_no);
            self.slices.insert(slice_no, slice);
        }
        let slice = self.slices.get_mut(&slice_no).unwrap();
        slice
    }
}

fn test(buff: &mut [u8]) -> Result<&mut [u8], &str> {
    let data = &mut buff[3..25];
    Ok(data)
}

fn main() {
    let mut man = Man::new();
    let mut slice = man.get_slice(1);
    slice.start = 5;

    println!("{:?}", slice);
    let mut slice1 = man.get_slice(2);
    println!("{:?}", slice1);
    slice1.end = 5;
    println!("{:?}", slice1);

    let mut slice2 = man.get_slice(2);
    println!("{:?}", slice2);

    let mut buf = [
        1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 4, 8, 9, 10, 11, 12, 13, 14, 15,
        16, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
    ]; // From PackedSize
    {
        println!("buf {:?}", &buf);
        if let Ok(data) = test(&mut buf) {
            println!("data {:?}", &data);
        }
        println!("buf {:?}", &buf);
    }
    let mut vbuf = buf.to_vec();
    println!("b vec buf {:?}", &vbuf);
    let (left, right) = vbuf.split_at_mut(19);
    println!("a vec buf {:?}", &left);
    println!("remain buf {:?}", &right);
    vbuf = right.to_vec();
    println!("b vec buf {:?}", &vbuf);
    
}
