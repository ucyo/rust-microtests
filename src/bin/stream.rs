use std::fs;
use std::fs::File;
use std::env;
use std::io::BufReader;
use std::io::prelude::*;

const BUF: usize = 4096;

fn main() {
    let filename = env::args().nth(1).expect("");
    let filesize = fs::metadata(&filename).expect("Could not read").len() as usize;
    if &filesize % 4 != 0 {
        panic!("{:?} is not 4 bytes long.", filesize);
    }
    let reads = filesize / BUF;

    let file = File::open(filename).expect("Failed to open file");
    let mut buf_reader = BufReader::new(file);
    let mut buffer: Vec<u8> = Vec::with_capacity(BUF);
    unsafe {
        buffer.set_len(BUF);
    }
    for _ in 0..reads {
        buf_reader.read_exact(&mut buffer).expect("Could not fill buffer");
        // do stuff with buffer
    }
}
