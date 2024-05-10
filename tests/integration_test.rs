use std::path::Path;

use ilmarinen::WordSmith;


#[test]
fn debug_test() {
    let tt = ilmarinen::WordSmith::test_main(Path::new("data/"));
}
