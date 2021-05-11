use std::fs::File;
use std::io::{self, prelude::*};

use durable_file::DurableFile;

fn main() -> Result<(), io::Error> {
    let file = File::create("hello.txt")?;
    // file.write_all(b"Hello, world!")?;

    let mut durable_file = DurableFile::new(file);
    durable_file.write_all(b"Hello, world!")?;
    durable_file.close()?;

    println!("Durable file: {:?}", durable_file);

    Ok(())
}
