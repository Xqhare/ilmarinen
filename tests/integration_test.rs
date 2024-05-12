use std::path::Path;

use ilmarinen::WordSmith;


#[test]
fn debug_test() {
    let tt = WordSmith::new(Path::new("data/"));
    println!("FINAL RESULT : {:?}", tt);
    assert!(tt.is_ok())
}
