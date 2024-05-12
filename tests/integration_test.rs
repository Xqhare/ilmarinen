use std::path::Path;

use ilmarinen::{WordSmith, MintingType};


#[test]
fn debug_test() {
    let tt = WordSmith::test_main(MintingType::People);
    let tm = WordSmith::test_main(MintingType::ShipName);
    println!("FINAL RESULT : {:?} | {:?}", tt, tm);
}

#[test]
fn new_wordsmith_test() { 
    let tt = WordSmith::new(Path::new("data/"));
    //println!("FINAL RESULT : {:?}", tt);
    assert!(tt.is_ok())
}
