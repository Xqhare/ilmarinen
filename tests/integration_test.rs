use std::path::Path;

use ilmarinen::{WordSmith, MintingType};

#[test]
fn mint_place_name_test() {
    let mut word_smith = WordSmith::new(Path::new("data/")).unwrap();
    let people = word_smith.mint(MintingType::People, 100000);
    assert!(people.is_ok());
    let place = word_smith.mint(MintingType::PlaceName, 100000);
    assert!(place.is_ok());
    for entry in place.unwrap().result {
        println!("{}", entry)
    };
}

