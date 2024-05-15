
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
    sleep_duration: Arc<Duration>,
}

impl WordSmith {
    /// Takes in a folder path. It extends for the needed files on its own, they need to be present
    /// in the supplied path though.
    pub fn new(data_path: &Path) -> Result<WordSmith, Error> {
        if data_path.is_dir() {
            if let Ok(answ) = data_path.try_exists() {
                let dur = Arc::new(Duration::from_micros(3300));
                match answ {
                    true => { 
                        match UnitArchipelago::new(data_path) {
                            Ok(unit_archipelago) => Ok( WordSmith { unit_archipelago: unit_archipelago.into(), result: Arc::new(Mutex::new(MintingResult::default())), sleep_duration: dur } ),
                            Err(error) => Err(Error::new(ErrorKind::Other, error))
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
    pub fn mint(&mut self, minting_type: MintingType, mint_amount: usize) -> Result<MintingResult, Error> {
        if mint_amount > 0 {
                let thread_pool = ThreadPool::provision_thread_pool(mint_amount);
                if let Ok(pool) = thread_pool {
                    self.result = Default::default();
                    for n in 0..mint_amount {
                        let data = self.unit_archipelago.clone();
                        let out = self.result.clone();
                        let sleep_duration = self.sleep_duration.clone();
                        let sleep_offset = {
                            if n < 3300 {
                                n 
                            } else {
                                n % 3300
                            }
                        };
                        match minting_type {
                            MintingType::PlaceName => pool.execute(move||{mint_place(data, out, sleep_duration, sleep_offset.try_into().expect("Sleep Offset is larger than u64!"))}),
                            MintingType::People => pool.execute(move||{mint_people(data, out, sleep_duration, sleep_offset.try_into().expect("Sleep Offset is larger than u64!"))}),
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
                        // Sleeping for one millisecond means 4.2 million cpu cycles for
                        // a 4.2 Ghz cpu.
                        // Sleeping for 100 microseconds would allow 42k cpu cycles.
                        //
                        // After some very sophisticated testing, 3300 micros seems to be near the
                        // sweetspot.
                        // After even more testing, this is better by a second! Still bad, but
                        // hand-rolled!
                        if mint_amount < 3300 {
                            thread::sleep(*self.sleep_duration.clone());
                        } else {
                            thread::sleep(self.sleep_duration.clone().saturating_add(Duration::from_micros((mint_amount % 22000) as u64)))
                        }
                        if let Ok(store) = self.result.try_lock() {
                            if store.result.len() == mint_amount {
                                return Ok(store.clone());
                            }
                        };
                        //println!("BLOCKED MAIN");
                    }
                } else {
                    Err(Error::other("Fatal runtime error, unable to create thread pool."))
                }
        } else {
            Err(Error::other(format!("mint_amount '{}' is less than 1!", mint_amount)))
        }
    }

    pub fn test_main(minting_type: MintingType) {
        println!("{:?}", minting_type);
    }
}
