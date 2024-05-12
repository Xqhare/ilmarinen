# ilmarinen

An intermidiate library, creating random names and objects.

## Usage

To set up:
```rust
use ilmarinen::WordSmith;

fn main() {
    // The json libraries need to be located inside to supplied path.
    let word_smith = WordSmith::new("data/"));
}
```

There is no `WordSmith::default()`, the path containing the libraries must be supplied.
