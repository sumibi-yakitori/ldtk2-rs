//! A thin crate for people who just want to use ldtk files freely.
//!
//!
//! # Why did I create this nonsense?
//!
//! - [LDtk-rs](https://github.com/katharostech/LDtk-rs) <strike>uses code generation, it does not get autocomplete support from rust-analyzer.</strike> Also, there are [special license](https://github.com/katharostech/katharos-license) restrictions on using that crate.
//! - [ldtk_rust](https://github.com/estivate/ldtk_rust) uses `.except()` inside the crate, you can't handle errors.
//!
//!
//! # Supported LDtk file versions
//!
//! `^1.1.3`
//!
//!
//! # Usage
//!
//! ```sh
//! cargo add ldtk2
//! ```
//!
//! ```rust
//! use std::{error::Error, path::Path, convert::TryInto};
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

// pub use anyhow;
pub use ldtk::*;
pub use serde_json;
pub type Ldtk = LdtkJson;

// use anyhow::{Context, Error, Result};
use std::{convert::TryFrom, path::Path};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
  #[error(transparent)]
  Io(#[from] std::io::Error),
  #[error(transparent)]
  SerdeJson(#[from] serde_json::Error),
}

impl Ldtk {
  pub fn from_path(path: impl AsRef<Path>) -> Result<Self, Error> {
    // let path_str = path.as_ref().to_string_lossy().to_string();
    Ok(serde_json::from_str(
      &std::fs::read_to_string(path)
        .map_err(|e| Error::Io(e))
        // .with_context(|| format!("Failed to open ldtk file: {}", path_str))
        ?,
    )?)
  }

  pub fn from_str(s: impl AsRef<str>) -> Result<Self, serde_json::Error> {
    Ok(serde_json::from_str(s.as_ref())?)
  }
}

impl TryFrom<&Path> for Ldtk {
  type Error = Error;

  fn try_from(path: &Path) -> Result<Self, Self::Error> {
    Ldtk::from_path(path)
  }
}

impl TryFrom<&str> for Ldtk {
  type Error = serde_json::Error;

  fn try_from(s: &str) -> Result<Self, Self::Error> {
    Ldtk::from_str(s)
  }
}
