# Ilmarinen: Random Name and Concept Generation

Ilmarinen is an intermediate-level Rust library designed to provide a convinient, lightweigt and maintainable way to generate random names and concepts for various purposes, including but not limited to:

- Game Development (names for characters, locations and more)
- Writing (creating fictional elements, currencies, empires and more)
- Testing (generating a large amount of data for test purposes)
- Anything else you can imagine!

If you want a gui application, my [randomiserProject](https://github.com/Xqhare/randomiserProject) written in python. It should also run on windows, while Ilmarinen's implementation makes it Unix only.
On that note, Ilmarinen is just a rewrite of that project in rust with badly handrolled multi-threading. It handles a few ten-thousand requests with grace, but the performance does gegin to drop if you move beyond that. Maybe I will rewrite that part once I actually know what I am doing, if I ever reach that point :).

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
- Self-rolled Multi-Threading
- Utilises an actual CSPRNG, meaning that the data is guaranteed to be random

### Further Development

- Expaning the range of name and concept types (e.g. mythical creatures, organisations, companies, game titles, video ideas)
- Make multi-threading work gooder

## Inspired by Ancient Gods

The library's name, Ilmarinen, is drawn from the finnish god of the air, weather and craftsmanship. In the Kalevala, Ilmarinen is renowned for his skill in forging magical objects, feflecting the library's ability to craft random names and concepts.

## Usage

### Cargo.toml

To set up include it in your dependencies:
```toml
ilmarinen = {git = "https://github.com/Xqhare/ilmarinen" }
```
Then run `cargo update`.

### main.rs

In your `main.rs` you first construct the `WordSmith` by suppling it the path to the directory where the json libraries inside the `data` directory of this repo are on your system.
Then it's just a matter of invoking `word_smit.mint(minting_type, mint_amount)` with a `MintingType` representing the type you are requesting, and `mint_amount` being the amount of results you want. Please not that suppling a `0` will produce an `Err()`. Only positive Integers above `0` and below `usize` are permitted.
```rust
use ilmarinen::WordSmith;

fn main() {
    // The json libraries need to be located inside to supplied path.
    let word_smith = WordSmith::new("data/"));
    // You can supply any `MintingType` you choose, as well as any amount. As long as its larger than 0 and smaller than an u64.
    let operation = word_smith.mint(MintingType::Operation, 100000);
    assert!(operation.is_ok());
    for entry in operation.unwrap().result {
        println!("{}", entry)
    };
}
```

There is no `WordSmith::default()`, the path containing the libraries must be supplied.

### `MintingType`

The following `MintingType`'s are supported:

- Place
- People
- Artifact
- Operation
- ShipName
- ShipClass
- Currency
- MetalAndAlloy
- Empire
- Government
- Language

## Making Ilmarinen your own

You can very easily edit the `json` files contained in this repo and change the output of the library completely, without having to touch the codebase. Go wild!
