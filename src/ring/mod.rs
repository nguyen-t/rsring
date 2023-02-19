mod ring;
mod functions;

pub use {
  ring::Ring,
};

#[cfg(test)]
mod ring_tests {
  use crate::ring::Ring;
  #[test]
  fn init_test() -> Result<(), String> {
    return match Ring::<[u64; 2], [u8; 0]>::new(0, 32) {
      Ok(_) => Ok(()),
      Err(err) => Err(err.to_string())
    };
  }
}