#![feature(c_size_t)]
mod io_uring;
mod util;
mod squeue;
mod cqueue;
mod ring;

pub use {
  squeue::SQueue,
  cqueue::CQueue,
  ring::RSRing,
};

#[cfg(test)]
mod rsring_tests {
  use crate::RSRing;
  #[test]
  fn init_test() -> Result<(), String> {

    return match RSRing::new(3) {
      Ok(_) => Ok(()),
      Err(err) => Err(err.to_string())
    };
  }
}