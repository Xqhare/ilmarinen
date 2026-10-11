use std::{
    io::{Error, ErrorKind},
    path::Path,
};

use mawu::XffValue;

pub fn read_json<P>(file_path: P) -> Result<XffValue, Error>
where
    P: AsRef<Path>,
{
    match mawu::read::json(file_path) {
        Ok(out) => Ok(out),
        Err(error) => Err(Error::new(ErrorKind::InvalidData, error)),
    }
}
