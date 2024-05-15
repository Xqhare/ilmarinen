
pub mod minting_type;
pub mod minting_result;
mod presses;

pub mod minters {
    use std::{sync::{Arc, Mutex}, thread, time::Duration};

    use crate::{unit_pools::UnitArchipelago, MintingResult};

    use super::presses::{people_press::people_press, place_press::place_press};

    pub fn mint_place(data: Arc<UnitArchipelago>, out: Arc<Mutex<MintingResult>>, sleep_duration: Arc<Duration>, sleep_offset: u64) {
        loop {
            if let Ok(mut store) = out.try_lock() {
                let result = place_press(data).expect("SOMETHING WENT TERRIBLY WRONG!");
                store.result.push(result);
                break;
            };
            //println!("MINT place BLOCKED");
            thread::sleep(sleep_duration.saturating_add(Duration::from_micros(sleep_offset)));
        }
    }

    pub fn mint_people(data: Arc<UnitArchipelago>, out: Arc<Mutex<MintingResult>>, sleep_duration: Arc<Duration>, sleep_offset: u64) {
        loop {
            if let Ok(mut store) = out.try_lock() {
                let result = people_press(data).expect("SOMETHING WENT TERRIBLY WRONG!");
                store.result.push(result);
                break;
            };
            //println!("MINT people BLOCKED");
            thread::sleep(sleep_duration.saturating_add(Duration::from_micros(sleep_offset)));
        }
    }

    pub fn mint_language(data: Arc<UnitArchipelago>, out: Arc<Mutex<MintingResult>>, sleep_duration: Arc<Duration>, sleep_offset: u64) {
        loop {
            if let Ok(mut store) = out.try_lock() {
                let result = people_press(data).expect("SOMETHING WENT TERRIBLY WRONG!");
                store.result.push(result);
                break;
            };
            //println!("MINT language BLOCKED");
            thread::sleep(sleep_duration.saturating_add(Duration::from_micros(sleep_offset)));
        }
    }

    pub fn mint_metal_alloy(data: Arc<UnitArchipelago>, out: Arc<Mutex<MintingResult>>, sleep_duration: Arc<Duration>, sleep_offset: u64) {
        loop {
            if let Ok(mut store) = out.try_lock() {
                let result = people_press(data).expect("SOMETHING WENT TERRIBLY WRONG!");
                store.result.push(result);
                break;
            };
            //println!("MINT metal_alloy BLOCKED");
            thread::sleep(sleep_duration.saturating_add(Duration::from_micros(sleep_offset)));
        }
    }

    pub fn mint_artifact(data: Arc<UnitArchipelago>, out: Arc<Mutex<MintingResult>>, sleep_duration: Arc<Duration>, sleep_offset: u64) {
        loop {
            if let Ok(mut store) = out.try_lock() {
                let result = people_press(data).expect("SOMETHING WENT TERRIBLY WRONG!");
                store.result.push(result);
                break;
            };
            //println!("MINT artifact BLOCKED");
            thread::sleep(sleep_duration.saturating_add(Duration::from_micros(sleep_offset)));
        }
    }

    pub fn mint_operation(data: Arc<UnitArchipelago>, out: Arc<Mutex<MintingResult>>, sleep_duration: Arc<Duration>, sleep_offset: u64) {
        loop {
            if let Ok(mut store) = out.try_lock() {
                let result = people_press(data).expect("SOMETHING WENT TERRIBLY WRONG!");
                store.result.push(result);
                break;
            };
            //println!("MINT operation BLOCKED");
            thread::sleep(sleep_duration.saturating_add(Duration::from_micros(sleep_offset)));
        }
    }
}

