use std::{path::Path, fs::File, io::{Error, Read, ErrorKind}};

use json::{JsonValue, parse};

use crate::WordSmith;

//use crate::WordSmith;

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

pub fn construct_word_smith(data_path: &Path) -> Result<WordSmith, Error>{
    let gen_lib_path = data_path.join("gen_lib.json");
    if let Ok(data) = read_json(gen_lib_path) {
        for thing in data.entries() {
            for entry in thing.1.entries() {
                println!("{:?}", entry);
            }
        }
    } else {
        println!("PANIK!");
    }
}
    
