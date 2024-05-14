use std::path::Path;

use ilmarinen::{WordSmith, MintingType};

#[test]
fn mint_place_name_test() {
    let mut tmp = WordSmith::new(Path::new("data/")).unwrap();
    let tmp1 = tmp.mint(MintingType::People, 100000);
    /* for entry in tmp1.unwrap().result {
        println!("{}", entry)
    } */
    assert!(tmp1.is_ok())
}

