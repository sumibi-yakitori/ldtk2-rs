[![Crates.io](https://img.shields.io/crates/v/ldtk2.svg)](https://crates.io/crates/ldtk2)
[![Documentation](https://docs.rs/ldtk2/badge.svg)](https://docs.rs/ldtk2)
[![License](https://img.shields.io/crates/l/ldtk2.svg)](LICENSE)
[![Workflow Status](https://github.com/sumibi-yakitori/ldtk2-rs/workflows/Rust/badge.svg)](https://github.com/sumibi-yakitori/ldtk2-rs/actions?query=workflow%3A%22Rust%22)

# ldtk2

A thin crate for people who just want to use ldtk files freely.


## Usage

```sh
cargo add ldtk2
```

```rust
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
  use ldtk2::Ldtk;

  let map = Ldtk::from_file("tests/example.ldtk")?;
  // or
  let map = Ldtk::from_str(include_str!("../tests/example.ldtk"))?;

  Ok(())
}
```


## Why did I create this nonsense?

- [LDtk-rs](https://github.com/katharostech/LDtk-rs) uses code generation, it does not get autocomplete support from rust-analyzer. Also, there are [special license](https://github.com/katharostech/katharos-license) restrictions on using that crate.
- [ldtk_rust](https://github.com/estivate/ldtk_rust) uses `.except()` inside the crate, you can't handle the error.
