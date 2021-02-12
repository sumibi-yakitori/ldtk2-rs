# ldtk2-rs


## 

```rust
  use ldtk2::Ldtk;

  let map = Ldtk::from_file("example.ldtk")?;
  // or
  let map = Ldtk::from_str(include_str!("example.ldtk"))?;
```

# Why did I create this?

- Because [LDtk-rs](https://github.com/katharostech/LDtk-rs) uses code generation, it does not get autocomplete support from rust-analyzer. Also, there are special license restrictions on using that crate.
- [ldtk_rust](https://github.com/estivate/ldtk_rust) uses `.except()` in its crate, so it cannot handle errors.
