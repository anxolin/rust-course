use std::fs::File;
use std::io::prelude::*;

use durable_file::DurableFile;

fn main() {
    let file = File::create("hello.txt").unwrap();
    // file.write_all(b"Hello, world!")?;

    let mut durable_file = DurableFile::new(file);
    println!("Durable file: {:?}", durable_file);

    durable_file.write_all(b"Hello, world!").unwrap();
    durable_file.close().unwrap();
}
