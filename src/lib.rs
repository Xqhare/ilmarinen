mod jisard;
mod minting;
mod unit_pools;

use std::{
    io::{Error, ErrorKind},
    path::Path,
};

pub use crate::minting::minting_result::MintingResult;
pub use crate::minting::minting_type::MintingType;
use fortuna::{Fortuna, RngApi};
use minting::minters::{
    mint_artifact, mint_currency, mint_empire, mint_government, mint_language, mint_metal_alloy,
    mint_operation, mint_people, mint_place, mint_ship_class, mint_ship_name,
};
use thoth::Thoth;
use tyche::Tyche;
use unit_pools::UnitArchipelago;

pub struct WordSmith {
    unit_archipelago: UnitArchipelago,
    rng: Fortuna,
    unicode: Thoth,
}

impl WordSmith {
    /// Takes in a folder path. It extends for the needed files on its own, they need to be present
    /// in the supplied path though.
    ///
    /// This will seed the random number generator with a cryptographically secure random number.
    ///
    /// Use this if you don't need or want deterministic results.
    pub fn new(data_path: &Path) -> Result<WordSmith, Error> {
        let seed = {
            let mut tyche = match Tyche::new() {
                Ok(tyche) => tyche,
                Err(error) => return Err(Error::new(ErrorKind::Other, error)),
            };
            tyche
                .random_bytes(64)
                .map_err(|_| Error::new(ErrorKind::Other, "Error generating seed"))?
        };
        WordSmith::new_seeded(data_path, seed)
    }

    /// Takes in a folder path and a seed.
    ///
    /// The provided seed is used to seed the random number generator.
    /// Using the same seed will result in the same generated words.
    ///
    /// Use this if deterministic results are desired.
    pub fn new_seeded<V: Into<Vec<u8>>>(data_path: &Path, seed: V) -> Result<WordSmith, Error> {
        if data_path.is_dir()
            && let Ok(answ) = data_path.try_exists()
            && answ
        {
            match UnitArchipelago::new(data_path) {
                Ok(unit_archipelago) => Ok(WordSmith {
                    unit_archipelago,
                    rng: Fortuna::new(seed),
                    unicode: Thoth::new().map_err(|error| Error::new(ErrorKind::Other, error))?,
                }),
                Err(error) => Err(Error::other(error)),
            }
        } else {
            Err(Error::other(format!(
                "Invalid data. '{data_path:?}' needs to exist and be a directory."
            )))
        }
    }

    pub fn mint(
        &mut self,
        minting_type: MintingType,
        mint_amount: usize,
    ) -> Result<MintingResult, Error> {
        let mut out = MintingResult::new();
        if mint_amount > 0 {
            for _ in 0..mint_amount {
                let result = match minting_type {
                    MintingType::Place => mint_place(&self.unit_archipelago, &mut self.rng),
                    MintingType::People => mint_people(&self.unit_archipelago, &mut self.rng),
                    MintingType::Artifact => mint_artifact(&self.unit_archipelago, &mut self.rng),
                    MintingType::Operation => mint_operation(&self.unit_archipelago, &mut self.rng),
                    MintingType::ShipName => mint_ship_name(&self.unit_archipelago, &mut self.rng),
                    MintingType::ShipClass => {
                        mint_ship_class(&self.unit_archipelago, &mut self.rng)
                    }
                    MintingType::Currency => {
                        mint_currency(&self.unit_archipelago, &mut self.rng, &self.unicode)
                    }
                    MintingType::MetalAndAlloy => {
                        mint_metal_alloy(&self.unit_archipelago, &mut self.rng)
                    }
                    MintingType::Empire => mint_empire(&self.unit_archipelago, &mut self.rng),
                    MintingType::Government => {
                        mint_government(&self.unit_archipelago, &mut self.rng)
                    }
                    MintingType::Language => mint_language(&self.unit_archipelago, &mut self.rng),
                };
                let result = result?;
                out.add_result(result);
            }
        }
        Ok(out)
    }
}
