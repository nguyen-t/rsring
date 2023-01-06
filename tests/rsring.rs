extern crate rsring;

#[cfg(test)]
mod tests {
  #[test]
  fn rsring_init() -> Result<(), String> {
    return match rsring::RSRing::new(3) {
      Ok(_) => Ok(()),
      Err(err) => Err(err.to_string())
    };
  }
}