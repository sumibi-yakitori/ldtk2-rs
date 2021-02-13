//! A thin crate for people who just want to use ldtk files freely.
//!
//!
//! # Why did I create this nonsense?
//!
//! - [LDtk-rs](https://github.com/katharostech/LDtk-rs) uses code generation, it does not get autocomplete support from rust-analyzer. Also, there are [special license](https://github.com/katharostech/katharos-license) restrictions on using that crate.
//! - [ldtk_rust](https://github.com/estivate/ldtk_rust) uses `.except()` inside the crate, you can't handle errors.
//!
//!
//! # Supported LDtk file versions
//!
//! `>=0.7.2`
//!
//!
//! # Usage
//!
//! ```sh
//! cargo add ldtk2
//! ```
//!
//! ```rust
//! use std::error::Error;
//! use std::path::Path;
//! use std::convert::TryInto;
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!   use ldtk2::Ldtk;
//!
//!   let map = Ldtk::from_path("tests/example.ldtk")?;
//!   // or
//!   let map: Ldtk = Path::new("tests/example.ldtk").try_into()?;
//!   // or
//!   let map = Ldtk::from_str(include_str!("../tests/example.ldtk"))?;
//!   // or
//!   let map: Ldtk = include_str!("../tests/example.ldtk").try_into()?;
//!
//!   Ok(())
//! }
//! ```

mod ldtk;
pub use ldtk::*;
use std::{convert::TryFrom, error::Error, path::Path};
pub type Ldtk = Coordinate;

impl Ldtk {
  pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
    let value: Self = serde_json::from_str(&std::fs::read_to_string(path)?)?;
    Ok(value)
  }

  pub fn from_str<S: AsRef<str>>(s: S) -> Result<Self, Box<dyn Error>> {
    let value: Self = serde_json::from_str(s.as_ref())?;
    Ok(value)
  }
}

impl TryFrom<&Path> for Ldtk {
  type Error = Box<dyn Error>;

  fn try_from(path: &Path) -> Result<Self, Self::Error> {
    Ldtk::from_path(path)
  }
}

impl TryFrom<&str> for Ldtk {
  type Error = Box<dyn Error>;

  fn try_from(s: &str) -> Result<Self, Self::Error> {
    Ldtk::from_str(s)
  }
}
