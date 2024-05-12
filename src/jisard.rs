use std::{path::Path, fs::File, io::{Error, Read, ErrorKind}};

use json::{JsonValue, parse};

pub fn read_json<P>(file_path: P) -> Result<JsonValue, Error> where P: AsRef<Path> {
    let mut input = File::open(file_path)?;
    let mut buffer: String = Default::default();
    let _ = input.read_to_string(&mut buffer);
    let out = parse(&buffer);
    if out.is_ok() {
        Ok(out.expect("Is Ok checked!"))
    } else {
        Err(Error::new(ErrorKind::InvalidData, out.err().unwrap()))
    }
}

