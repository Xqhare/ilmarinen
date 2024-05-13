
pub mod minting_type;
pub mod minting_result;

pub mod minters {
    use std::{sync::{Arc, Mutex}, thread, time::Duration};

    use crate::{unit_pools::UnitArchipelago, MintingResult};

    pub fn mint_place(data: Arc<UnitArchipelago>, out: Arc<Mutex<MintingResult>>) {
        loop {
            if let Ok(mut store) = out.try_lock() {
                store.result.push("test".to_string());
                break;
            };
            thread::sleep(Duration::from_millis(5));
        }
            
    }

    pub fn mint_people(data: Arc<UnitArchipelago>, out: Arc<Mutex<MintingResult>>) {
        todo!("implement this")
    }
}
