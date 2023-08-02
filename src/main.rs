use std::io;
use std::io::Write; // <--- bring flush() into scope
use std::thread;
use std::time::Duration;

fn main() {
    let mut data = vec![0u8; 98];

    let mut iter = data.chunks(10);
    while let Some(mut chunk) = iter.next() {
        println!("len = {}, {:?}", chunk.len(), chunk);
    }
}