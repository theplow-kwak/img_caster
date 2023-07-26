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
}
