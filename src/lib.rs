
mod jisard;
mod unit_pools;
mod minting_type;

use std::{io::{Error, ErrorKind}, path::Path};

pub use minting_type::MintingType;
use unit_pools::UnitArchipelago;


#[derive(Debug, Clone)]
pub struct WordSmith {
    lexical_unit_lagoon: UnitArchipelago,
}

impl WordSmith {
    /// Takes in a folder path. It extends for the needed files on its own, they need to be present
    /// in the supplied path though.
    pub fn new(data_path: &Path) -> Result<WordSmith, Error> {
        // I only check the path here, real logic is external in `construct_self()`.
        if data_path.is_dir() {
            if let Ok(answ) = data_path.try_exists() {
                match answ {
                    true => { 
                        match UnitArchipelago::new(data_path) {
                            Ok(unit_archipelago) => { Ok( WordSmith { lexical_unit_lagoon: unit_archipelago } ) },
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
/* 
    pub fn mint(minting_type: MintingType, mint_amount: usize) -> MintingResult {
        unimplemented!();
    } */

    pub fn test_main(minting_type: MintingType) {
        println!("{:?}", minting_type);
    }
}
