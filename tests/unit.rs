mod tests {
  use rsring::RSRing;
  
  #[test]
  fn rsring_init() -> Result<(), String> {

    return match RSRing::new(3) {
      Ok(_) => Ok(()),
      Err(err) => Err(err.to_string())
    };
  }
}