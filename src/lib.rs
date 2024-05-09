
mod json;
mod lexical_unit_pool;
mod lexical_unit_lagoon;

use std::io::{Error, ErrorKind};
use std::path::Path;

use crate::json::{read_json, construct_word_smith};

use crate::lexical_unit_pool::LexicalUnitPool;


#[derive(Debug, Clone)]
pub struct WordSmith {
    lexical_unit_lagoon: LexicalUnitLagoon,
}

impl WordSmith {
    /// Takes in a folder path. It extends for the needed files on its own, they need to be present
    /// in the supplied path though.
    pub fn new(data_path: &Path) -> Result<WordSmith, Error> {
        // I only check the path here, real logic is external in `construct_self()`.
        if data_path.is_dir() {
            let tmp = data_path.try_exists();
            if let Ok(answ) = data_path.try_exists() {
                match answ {
                    true => { 
                        match construct_word_smith(data_path) {
                            Ok(word_smith) => { Ok(word_smith) },
                            Err(error) => { Err(Error::new(ErrorKind::Other, error)) }
                        }
                    }
                    false => { Err(Error::from(ErrorKind::NotFound)) },
                }
            } else {
                Err(Error::from(ErrorKind::PermissionDenied))
            }
        } else {
            Err(Error::other(format!("Invalid data. '{:?}' needs to exist and be a directory.", data_path)))
        } 
    }

    pub fn test_main() {
        let a = read_json("data/gen_lib.json");
        println!("{:?}", a);
    }
}
