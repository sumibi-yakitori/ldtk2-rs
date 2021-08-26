#[cfg(test)]
mod tests {
  use std::{convert::TryInto, error::Error, path::Path};

  #[test]
  fn basic_test() -> Result<(), Box<dyn Error>> {
    use ldtk2::Ldtk;

    let _map = Ldtk::from_path("tests/example.ldtk")?;
    // or
    let _map: Ldtk = Path::new("tests/example.ldtk").try_into()?;
    // or
    let _map = Ldtk::from_str(include_str!("../tests/example.ldtk"))?;
    // or
    let _map: Ldtk = include_str!("../tests/example.ldtk").try_into()?;

    Ok(())
  }

  #[test]
  fn send_error_test() {
    let e = ldtk2::Error::Io(std::io::Error::from(std::io::ErrorKind::NotFound));
    let handle = std::thread::spawn(move || {
      drop(e);
    });
    assert!(handle.join().is_ok());
  }
}
