mod ldtk;
pub use ldtk::*;
use std::{error::Error, path::Path};
pub type Ldtk = Coordinate;

impl Ldtk {
  pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
    let value: Self = serde_json::from_str(&std::fs::read_to_string(path)?)?;
    Ok(value)
  }

  pub fn from_str<S: AsRef<str>>(s: S) -> Result<Self, Box<dyn Error>> {
    let value: Self = serde_json::from_str(s.as_ref())?;
    Ok(value)
  }
}
