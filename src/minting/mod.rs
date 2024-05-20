
pub mod minting_type;
pub mod minting_result;
mod presses;

pub mod minters {
    use std::{sync::{Arc, Mutex}, thread, time::Duration};

    use crate::{unit_pools::UnitArchipelago, MintingResult};

    use super::presses::{people_press::people_press, place_press::place_press, language_press::language_press, metal_alloy_press::metal_alloy_press, artifact_press::artifact_press, operation_press::operation_press, government_press::government_press, empire_press::empire_press, ship_name_press::ship_name_press};

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
                let result = language_press(data).expect("SOMETHING WENT TERRIBLY WRONG!");
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
                let result = metal_alloy_press(data).expect("SOMETHING WENT TERRIBLY WRONG!");
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
                let result = artifact_press(data).expect("SOMETHING WENT TERRIBLY WRONG!");
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
                let result = operation_press(data).expect("SOMETHING WENT TERRIBLY WRONG!");
                store.result.push(result);
                break;
            };
            //println!("MINT operation BLOCKED");
            thread::sleep(sleep_duration.saturating_add(Duration::from_micros(sleep_offset)));
        }
    }

    pub fn mint_government(data: Arc<UnitArchipelago>, out: Arc<Mutex<MintingResult>>, sleep_duration: Arc<Duration>, sleep_offset: u64) {
        loop {
            if let Ok(mut store) = out.try_lock() {
                let result = government_press(data, "").expect("SOMETHING WENT TERRIBLY WRONG!");
                store.result.push(result);
                break;
            };
            //println!("MINT government BLOCKED");
            thread::sleep(sleep_duration.saturating_add(Duration::from_micros(sleep_offset)));
        }
    }

    pub fn mint_empire(data: Arc<UnitArchipelago>, out: Arc<Mutex<MintingResult>>, sleep_duration: Arc<Duration>, sleep_offset: u64) {
        loop {
            if let Ok(mut store) = out.try_lock() {
                let result = empire_press(data).expect("SOMETHING WENT TERRIBLY WRONG!");
                store.result.push(result);
                break;
            };
            //println!("MINT government BLOCKED");
            thread::sleep(sleep_duration.saturating_add(Duration::from_micros(sleep_offset)));
        }
    }

    pub fn mint_ship_name(data: Arc<UnitArchipelago>, out: Arc<Mutex<MintingResult>>, sleep_duration: Arc<Duration>, sleep_offset: u64) {
        loop {
            if let Ok(mut store) = out.try_lock() {
                let result = ship_name_press(data, "").expect("SOMETHING WENT TERRIBLY WRONG!");
                store.result.push(result);
                break;
            };
            //println!("MINT government BLOCKED");
            thread::sleep(sleep_duration.saturating_add(Duration::from_micros(sleep_offset)));
        }
    }

}

