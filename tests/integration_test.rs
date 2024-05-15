use std::path::Path;

use ilmarinen::{WordSmith, MintingType};

#[test]
fn word_smith_main_test() {
    let mut word_smith = WordSmith::new(Path::new("data/")).unwrap();
    let people = word_smith.mint(MintingType::People, 100000);
    assert!(people.is_ok());
    let place = word_smith.mint(MintingType::PlaceName, 100000);
    assert!(place.is_ok());
    let language = word_smith.mint(MintingType::Language, 100000);
    assert!(language.is_ok());
    for entry in language.unwrap().result {
        println!("{}", entry)
    };
}

