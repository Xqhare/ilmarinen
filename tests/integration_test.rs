use std::path::Path;

use ilmarinen::WordSmith;


#[test]
fn debug_test() {
    let tt = WordSmith::test_main(Path::new("data/"));
}
