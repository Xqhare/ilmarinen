# Ilmarinen: Random Name and Concept Generation

Ilmarinen is an intermediate-level Rust library designed to provide a convinient and efficient way to generate random names and concepts for various purposes, including but not limited to:

- Game Development (names for characters, locations and more)
- Writing (creating fictional elements, currencies, empires and more)
- Testing (generating a large amount of data for test purposes)
- Anything else you can imagine!

## Features and Roadmap for 1.0.0

- Diverse Name and Concept Types:
    - People
    - Places
    - Artifacts
    - Operation names
    - Ships
    - Currencies
    - Metals and Alloys
    - Empires
    - Governments
    - Languages
    - Numbers
- Multi-Threading Support
- Utilises an actual CSPRNG, meaning that the data is guaranteed to be random

### Further Development

- Expaning the range of name and concept types (e.g. mythical creatures, organisations, companies, game titles, video ideas)

## Inspired by Ancient Gods

The library's name, Ilmarinen, is drawn from the Finnish god of the air, weather and craftsmanship. In the Kalevala, Ilmarinen is renowned for his skill in forging magical objects, feflecting the library's ability to craft random names and concepts.

## Multi-Threaded Performance

Ilmarinen leverages multi-threading to enhance its perfomance, particularly when generating large quantities of random names or concepts. This can significantly speed up tasks in applications that require large amounts of random data at high speed, like game development or testing.

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
