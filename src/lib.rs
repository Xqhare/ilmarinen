
mod jisard;
mod unit_pools;
mod minting;
mod thread_pool;

use std::{io::{Error, ErrorKind}, path::Path, sync::{Arc, Mutex}, thread, time::Duration};

pub use crate::minting::minting_type::MintingType; 
pub use crate::minting::minting_result::MintingResult;
use minting::minters::*;
use thread_pool::ThreadPool;
use unit_pools::UnitArchipelago;


#[derive(Debug, Clone)]
pub struct WordSmith {
    unit_archipelago: Arc<UnitArchipelago>,
    result: Arc<Mutex<MintingResult>>,
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
                            Ok(unit_archipelago) => { Ok( WordSmith { unit_archipelago: unit_archipelago.into(), result: Arc::new(Mutex::new(MintingResult::default())) } ) },
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

    /// Will only mint if mint_amount is larger than 0.
    pub fn mint(&self, minting_type: MintingType, mint_amount: usize) -> Result<(), Error> {
        if mint_amount > 0 {
                let thread_pool = {
                    if mint_amount <= 10 {
                        ThreadPool::build(2)
                    } else if mint_amount <= 50 {
                        ThreadPool::build(10)
                    } else if mint_amount <= 100 {
                        ThreadPool::build(20)
                    } else {
                        let size = {
                            if mint_amount / 5 > 250 {
                                250
                            } else {
                                mint_amount / 5
                            }
                        };
                        ThreadPool::build(size)
                    }
                };
                if let Ok(pool) = thread_pool {
                    for _n in 0..mint_amount {
                        let data = self.unit_archipelago.clone();
                        let out = self.result.clone();
                        match minting_type {
                            MintingType::PlaceName => pool.execute(|| {mint_place(data, out) }),
                            MintingType::People => pool.execute(|| {mint_people(data, out)}),
                            MintingType::Artifact => {},
                            MintingType::Operation => {},
                            MintingType::ShipName => {},
                            MintingType::ShipClass => {},
                            MintingType::Currency => {},
                            MintingType::MetalAndAlloy => {},
                            MintingType::Empire => {},
                            MintingType::Government => {},
                            MintingType::Language => {},
                            MintingType::Numbers => {},
                        }
                    }
                    // Don't have a better idea of waiting for all workers to finish.
                    // Could check the length of the created result, that idea seems worse.
                    loop {
                        // I really don't know how long this should be. Maybe 100,
                        // maybe 10, maybe mint_amount?
                        thread::sleep(Duration::from_millis(1));
                        if let Ok(store) = self.result.try_lock() {
                            if store.result.len() == mint_amount {
                                return Ok(());
                            }
                        };
                    }
                } else {
                    Err(Error::other("Fatal runtime error, unable to create thread pool."))
                }
        } else {
            Err(Error::other("Amount is less than 1!"))
        }
    }


    pub fn test_main(minting_type: MintingType) {
        println!("{:?}", minting_type);
    }
}
