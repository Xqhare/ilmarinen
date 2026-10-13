mod jisard;
mod minting;
mod unit_pools;

use std::{
    io::{Error, ErrorKind},
    path::Path,
    rc::Rc,
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
    rng: Option<Fortuna>,
    unicode: Rc<Thoth>,
}

impl WordSmith {
    /// Takes in a folder path. It extends for the needed files on its own, they need to be present
    /// in the supplied path though.
    ///
    /// This will create a `WordSmith` with externally managed random number generator and unicode
    /// libraries.
    ///
    /// Use this constructor to manage the random number generator and unicode libraries on
    /// your own.
    ///
    /// Requires the use of my own [thoth](https://github.com/xqhare/thoth) & [fortuna](https://github.com/xqhare/fortuna) libraries.
    ///
    /// # Example
    /// ```
    /// use std::{path::Path, rc::Rc};
    ///
    /// use fortuna::{Fortuna, RngApi};
    /// use thoth::Thoth;
    /// use ilmarinen::{MintingType, WordSmith};
    ///
    /// let thoth = Rc::new(Thoth::new().unwrap());
    /// let mut rng = Fortuna::new(vec![0, 1, 2]);
    /// let word_smith = WordSmith::new_managed(Path::new("data/"), thoth);
    ///
    /// assert!(word_smith.is_ok());
    /// let word_smith = word_smith.unwrap();
    ///
    /// // You need to call `mint_managed` afterwards instead of `mint`.
    ///
    /// let people = word_smith.mint_managed(&mut rng, MintingType::People, 10000);
    /// assert!(people.is_ok());
    /// let people = people.unwrap();
    /// let results = people.get_results();
    /// assert!(results.len() == 10000);
    ///
    /// // You can use the random number generator for anything else too!
    ///
    /// assert!(rng.random_u8().is_ok());
    /// ```
    pub fn new_managed(data_path: &Path, thoth: Rc<Thoth>) -> Result<WordSmith, Error> {
        if data_path.is_dir()
            && let Ok(answ) = data_path.try_exists()
            && answ
        {
            match UnitArchipelago::new(data_path) {
                Ok(unit_archipelago) => Ok(WordSmith {
                    unit_archipelago,
                    rng: None,
                    unicode: thoth,
                }),
                Err(error) => Err(Error::other(error)),
            }
        } else {
            Err(Error::other(format!(
                "Invalid data. '{data_path:?}' needs to exist and be a directory."
            )))
        }
    }
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
                    rng: Some(Fortuna::new(seed)),
                    unicode: Rc::new(
                        Thoth::new().map_err(|error| Error::new(ErrorKind::Other, error))?,
                    ),
                }),
                Err(error) => Err(Error::other(error)),
            }
        } else {
            Err(Error::other(format!(
                "Invalid data. '{data_path:?}' needs to exist and be a directory."
            )))
        }
    }

    /// Use if `WordSmith` was constructed via [`WordSmith::new_managed`].
    ///
    /// If you used [`WordSmith::new`], use [`WordSmith::mint`] instead.
    pub fn mint_managed(
        &self,
        rng: &mut Fortuna,
        minting_type: MintingType,
        mint_amount: usize,
    ) -> Result<MintingResult, Error> {
        mint_helper(
            &self.unit_archipelago,
            rng,
            &self.unicode,
            minting_type,
            mint_amount,
        )
    }

    /// Use if `WordSmith` was constructed via [`WordSmith::new`].
    ///
    /// If you used [`WordSmith::new_managed`], use [`WordSmith::mint_managed`] instead as
    /// calling this will error.
    pub fn mint(
        &mut self,
        minting_type: MintingType,
        mint_amount: usize,
    ) -> Result<MintingResult, Error> {
        if let Some(rng) = &mut self.rng {
            mint_helper(
                &self.unit_archipelago,
                rng,
                &self.unicode,
                minting_type,
                mint_amount,
            )
        } else {
            Err(Error::new(
                ErrorKind::Other,
                "No random number generator available",
            ))
        }
    }
}

fn mint_helper(
    unit_archipelago: &UnitArchipelago,
    rng: &mut Fortuna,
    unicode: &Rc<Thoth>,
    minting_type: MintingType,
    mint_amount: usize,
) -> Result<MintingResult, Error> {
    let mut out = MintingResult::new();
    if mint_amount > 0 {
        for _ in 0..mint_amount {
            let result = match minting_type {
                MintingType::Place => mint_place(unit_archipelago, rng),
                MintingType::People => mint_people(unit_archipelago, rng),
                MintingType::Artifact => mint_artifact(unit_archipelago, rng),
                MintingType::Operation => mint_operation(unit_archipelago, rng),
                MintingType::ShipName => mint_ship_name(unit_archipelago, rng),
                MintingType::ShipClass => mint_ship_class(unit_archipelago, rng),
                MintingType::Currency => mint_currency(unit_archipelago, rng, unicode),
                MintingType::MetalAndAlloy => mint_metal_alloy(unit_archipelago, rng),
                MintingType::Language => mint_language(unit_archipelago, rng),
                MintingType::Empire => mint_empire(unit_archipelago, rng),
                MintingType::Government => mint_government(unit_archipelago, rng),
            };
            out.add_result(result?);
        }
    }
    Ok(out)
}
