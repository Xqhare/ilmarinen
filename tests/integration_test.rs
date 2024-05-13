use std::path::Path;

use ilmarinen::{WordSmith, MintingType};


#[test]
fn debug_test() {
    let tt = WordSmith::test_main(MintingType::People);
    let tm = WordSmith::test_main(MintingType::ShipName);
    let t = WordSmith::test_main(MintingType::Empire);
    // println!("FINAL RESULT : {:?} | {:?} | {:?}", tt, tm, t);
}

#[test]
fn mint_place_name_test() {
    let mut tmp = WordSmith::new(Path::new("data/")).unwrap();
    let tmp1 = tmp.mint(MintingType::PlaceName, 100000);
}

