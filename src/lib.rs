mod jisard;
mod minting;
mod unit_pools;

use std::{path::Path, rc::Rc};

use crate::minting::minters::{
    mint_metal_alloy_object, mint_operation_object, mint_people_object, mint_place_object,
};
pub use crate::minting::minting_result::MintingResult;
pub use crate::minting::minting_type::{MintingObjectType, MintingType, ObjectResult};
use fortuna::{Fortuna, RngApi, RngResult};
use minting::minters::{
    mint_artifact, mint_currency, mint_empire, mint_government, mint_language, mint_metal_alloy,
    mint_operation, mint_people, mint_place, mint_ship_class, mint_ship_name,
};
use nemesis::NemesisError;
use thoth::Thoth;
use tyche::{RngError, Tyche};
use unit_pools::UnitArchipelago;

pub struct Ilmarinen {
    unit_archipelago: UnitArchipelago,
    rng: Option<Fortuna>,
    unicode: Rc<Thoth>,
}

impl Ilmarinen {
    /// Takes in a folder path. It extends for the needed files on its own, they need to be present
    /// in the supplied path though.
    ///
    /// This will create a `Ilmarinen` with externally managed random number generator and unicode
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
    /// use ilmarinen::{MintingType, Ilmarinen};
    ///
    /// let thoth = Rc::new(Thoth::new().unwrap());
    /// let mut rng = Fortuna::new(vec![0, 1, 2]);
    /// let word_smith = Ilmarinen::new_managed(Path::new("data/"), thoth);
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
    pub fn new_managed(data_path: &Path, thoth: Rc<Thoth>) -> RngResult<Ilmarinen> {
        if data_path.is_dir()
            && let Ok(answ) = data_path.try_exists()
            && answ
        {
            match UnitArchipelago::new(data_path) {
                Ok(unit_archipelago) => Ok(Ilmarinen {
                    unit_archipelago,
                    rng: None,
                    unicode: thoth,
                }),
                Err(error) => Err(error.into()),
            }
        } else {
            Err(RngError::Generic(format!(
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
    pub fn new(data_path: &Path) -> RngResult<Ilmarinen> {
        let seed = {
            let mut tyche = Tyche::new()?;
            tyche.random_bytes(64)?
        };
        Ilmarinen::new_seeded(data_path, seed)
    }

    /// Takes in a folder path and a seed.
    ///
    /// The provided seed is used to seed the random number generator.
    /// Using the same seed will result in the same generated words.
    ///
    /// Use this if deterministic results are desired.
    pub fn new_seeded<V: Into<Vec<u8>>>(data_path: &Path, seed: V) -> RngResult<Ilmarinen> {
        if data_path.is_dir()
            && let Ok(answ) = data_path.try_exists()
            && answ
        {
            let unit_archipelago = UnitArchipelago::new(data_path)?;
            Ok(Ilmarinen {
                unit_archipelago,
                rng: Some(Fortuna::new(seed)),
                unicode: Rc::new(Thoth::new()?),
            })
        } else {
            Err(RngError::Generic(format!(
                "Invalid data. '{data_path:?}' needs to exist and be a directory."
            )))
        }
    }

    /// Use if `Ilmarinen` was constructed via [`Ilmarinen::new_managed`].
    ///
    /// If you used [`Ilmarinen::new`], use [`Ilmarinen::mint`] instead.
    pub fn mint_managed(
        &self,
        rng: &mut Fortuna,
        minting_type: MintingType,
        mint_amount: usize,
    ) -> Result<MintingResult, RngError> {
        mint_helper(
            &self.unit_archipelago,
            rng,
            &self.unicode,
            minting_type,
            mint_amount,
        )
    }

    pub fn mint_managed_object(
        &self,
        rng: &mut Fortuna,
        minting_object_type: MintingObjectType,
    ) -> Result<ObjectResult, RngError> {
        mint_object_helper(
            &self.unit_archipelago,
            rng,
            &self.unicode,
            minting_object_type,
        )
    }

    /// Use if `Ilmarinen` was constructed via [`Ilmarinen::new`].
    ///
    /// If you used [`Ilmarinen::new_managed`], use [`Ilmarinen::mint_managed`] instead as
    /// calling this will error.
    pub fn mint(
        &mut self,
        minting_type: MintingType,
        mint_amount: usize,
    ) -> Result<MintingResult, RngError> {
        if let Some(rng) = &mut self.rng {
            mint_helper(
                &self.unit_archipelago,
                rng,
                &self.unicode,
                minting_type,
                mint_amount,
            )
        } else {
            Err(RngError::Generic(
                "No random number generator available".to_string(),
            ))
        }
    }
}

fn mint_object_helper(
    unit_archipelago: &UnitArchipelago,
    rng: &mut Fortuna,
    _unicode: &Rc<Thoth>,
    minting_object_type: MintingObjectType,
) -> Result<ObjectResult, RngError> {
    match minting_object_type {
        MintingObjectType::PlaceObject => mint_place_object(unit_archipelago, rng).map_err(|e| {
            RngError::Nemesis(NemesisError::new("Ilmarinen Object creation helper", e))
        }),
        MintingObjectType::PeopleObject(skills, trait_amount) => {
            mint_people_object(unit_archipelago, rng, skills, trait_amount).map_err(|e| {
                RngError::Nemesis(NemesisError::new("Ilmarinen Object creation helper", e))
            })
        }
        MintingObjectType::OperationObject => {
            mint_operation_object(unit_archipelago, rng).map_err(|e| {
                RngError::Nemesis(NemesisError::new("Ilmarinen Object creation helper", e))
            })
        }
        MintingObjectType::MetalAndAlloyObject => mint_metal_alloy_object(unit_archipelago, rng)
            .map_err(|e| {
                RngError::Nemesis(NemesisError::new("Ilmarinen Object creation helper", e))
            }),
    }
}

fn mint_helper(
    unit_archipelago: &UnitArchipelago,
    rng: &mut Fortuna,
    unicode: &Rc<Thoth>,
    minting_type: MintingType,
    mint_amount: usize,
) -> Result<MintingResult, RngError> {
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
