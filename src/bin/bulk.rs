use std::fs;
use std::fs::File;
use std::env;
use std::io::BufReader;
use byteorder::{ReadBytesExt, NativeEndian};

fn main() {
    let filename = env::args().nth(1).expect("");
    let filesize = fs::metadata(&filename).expect("Could not read").len() as usize;
    if &filesize % 4 != 0 {
        panic!("{:?} is not 4 bytes long.", filesize);
    }
    let float_array_size = filesize / 4;

    let file = File::open(filename).expect("Failed to open file");
    let mut buf_reader = BufReader::new(file);
    let mut buffer: Vec<f32> = Vec::with_capacity(float_array_size);
    unsafe {
        buffer.set_len(float_array_size);
    }
    buf_reader.read_f32_into::<NativeEndian>(&mut buffer[..]).expect("Failed");
}
