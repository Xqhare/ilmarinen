use std::path::Path;

use ilmarinen::{MintingType, WordSmith};

#[ignore = "reason"]
#[test]
fn word_smith_main_test() {
    let mut word_smith = WordSmith::new(Path::new("data/")).unwrap();
    let people = word_smith.mint(MintingType::People, 10000);
    assert!(people.is_ok());

    let place = word_smith.mint(MintingType::Place, 10000);
    assert!(place.is_ok());
    let metal_alloy = word_smith.mint(MintingType::MetalAndAlloy, 10000);
    assert!(metal_alloy.is_ok());
    let language = word_smith.mint(MintingType::Language, 10000);
    assert!(language.is_ok());
    let artifact = word_smith.mint(MintingType::Artifact, 10000);
    assert!(artifact.is_ok());
    let operation = word_smith.mint(MintingType::Operation, 10000);
    assert!(operation.is_ok());
    let government = word_smith.mint(MintingType::Government, 10000);
    assert!(government.is_ok());
    let empire = word_smith.mint(MintingType::Empire, 10000);
    assert!(empire.is_ok());
    let ship_name = word_smith.mint(MintingType::ShipName, 10000);
    assert!(ship_name.is_ok());
    let ship_class = word_smith.mint(MintingType::ShipClass, 10000);
    assert!(ship_class.is_ok());
    let currency = word_smith.mint(MintingType::Currency, 10000);
    assert!(currency.is_ok());
}

#[test]
fn peoples() {
    let mut word_smith = WordSmith::new(Path::new("data/")).unwrap();
    let people = word_smith.mint(MintingType::People, 10000);
    assert!(people.is_ok());
}

#[test]
fn places() {
    let mut word_smith = WordSmith::new(Path::new("data/")).unwrap();
    let places = word_smith.mint(MintingType::Place, 10000);
    assert!(places.is_ok());
}

#[test]
fn metal_alloys() {
    let mut word_smith = WordSmith::new(Path::new("data/")).unwrap();
    let metal_alloys = word_smith.mint(MintingType::MetalAndAlloy, 10000);
    assert!(metal_alloys.is_ok());
}

#[test]
fn languages() {
    let mut word_smith = WordSmith::new(Path::new("data/")).unwrap();
    let languages = word_smith.mint(MintingType::Language, 10000);
    assert!(languages.is_ok());
}

#[test]
fn artifacts() {
    let mut word_smith = WordSmith::new(Path::new("data/")).unwrap();
    let artifacts = word_smith.mint(MintingType::Artifact, 10000);
    assert!(artifacts.is_ok());
}

#[test]
fn operations() {
    let mut word_smith = WordSmith::new(Path::new("data/")).unwrap();
    let operations = word_smith.mint(MintingType::Operation, 10000);
    assert!(operations.is_ok());
}

#[test]
fn governments() {
    let mut word_smith = WordSmith::new(Path::new("data/")).unwrap();
    let governments = word_smith.mint(MintingType::Government, 10000);
    assert!(governments.is_ok());
}

#[test]
fn empires() {
    let mut word_smith = WordSmith::new(Path::new("data/")).unwrap();
    let empires = word_smith.mint(MintingType::Empire, 10000);
    assert!(empires.is_ok());
}

#[test]
fn ship_names() {
    let mut word_smith = WordSmith::new(Path::new("data/")).unwrap();
    let ship_names = word_smith.mint(MintingType::ShipName, 10000);
    assert!(ship_names.is_ok());
}

#[test]
fn ship_classes() {
    let mut word_smith = WordSmith::new(Path::new("data/")).unwrap();
    let ship_classes = word_smith.mint(MintingType::ShipClass, 10000);
    assert!(ship_classes.is_ok());
}

#[test]
fn currencies() {
    let mut word_smith = WordSmith::new(Path::new("data/")).unwrap();
    let currencies = word_smith.mint(MintingType::Currency, 10000);
    assert!(currencies.is_ok());
}
