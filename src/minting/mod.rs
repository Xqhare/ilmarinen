
pub mod minting_type;
pub mod minting_result;
mod presses;

pub mod minters {
    use std::{sync::{Arc, Mutex}, thread, time::Duration};

    use crate::{unit_pools::UnitArchipelago, MintingResult};

    use super::presses::people_press::people_press;

    pub fn mint_place(data: Arc<UnitArchipelago>, out: Arc<Mutex<MintingResult>>) {
        loop {
            if let Ok(mut store) = out.try_lock() {
                store.result.push("test".to_string());
                break;
            };
            thread::sleep(Duration::from_micros(100));
        }
    }

    pub fn mint_people(data: Arc<UnitArchipelago>, out: Arc<Mutex<MintingResult>>) {
        loop {
            if let Ok(mut store) = out.try_lock() {
                let result = people_press(data).expect("SOMETHING WENT TERRIBLY WRONG!");
                store.result.push(result);
                break;
            };
            thread::sleep(Duration::from_micros(100));
        }
    }
}
