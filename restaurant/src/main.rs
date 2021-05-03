use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;

fn main() {
    read_username_from_file()

}

fn read_username_from_file() -> Result<String, io::Error> {

    let mut s = String::new();
    File::open("test.txt")?.read_to_string(&mut s)?;
    Ok(s)

}
