//! A thin crate for people who just want to use ldtk files freely.
//!
//!
//! # Usage
//!
//! ```rust
//!   use ldtk2::Ldtk;
//!
//!   let map = Ldtk::from_file("example.ldtk")?;
//!   // or
//!   let map = Ldtk::from_str(include_str!("example.ldtk"))?;
//! ```
//!
//!
//! # Why did I create this nonsense?
//!
//! - [LDtk-rs](https://github.com/katharostech/LDtk-rs) uses code generation, it does not get autocomplete support from rust-analyzer. Also, there are special license restrictions on using that crate.
//! - [ldtk_rust](https://github.com/estivate/ldtk_rust) uses `.except()` inside the crate, you can't handle the error.

mod ldtk;
pub use ldtk::*;
use std::{error::Error, path::Path};
pub type Ldtk = Coordinate;

// #[test]
// fn test() {
//   use ldtk2::Ldtk;
//   let map = Ldtk::from_file("example.ldtk")?;
//   // or
//   let map = Ldtk::from_str(include_str!("example.ldtk"))?;
// }

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
