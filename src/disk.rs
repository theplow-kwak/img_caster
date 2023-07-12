use std::io;
use std::fs::OpenOptions;
use std::io::Read;

#[derive(Debug)]
pub struct DiskHandler {
    filename : String,
    rwflag : char,
    hfile : Option<std::fs::File>,
}

impl DiskHandler {
    pub fn new(filename: String, rwflag: char) -> DiskHandler {
        DiskHandler {filename, rwflag, hfile: None}
    }

    pub fn open(&mut self) -> &mut Self {
        let mut option = OpenOptions::new();
        if self.rwflag == 'w' {
            self.hfile = Some(option.write(true).create(true).open(&self.filename).expect("Failed to open physical drive"));
        } else {
            self.hfile = Some(option.read(true).open(&self.filename).expect("Failed to open physical drive"));
        }
        self
    }

    pub fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
    // Read data from the physical drive
        self.hfile.as_ref().expect("Invalid handle").read(buf)
    }

    pub fn write(&mut self) -> &mut Self {
        self
    }
}

