pub struct Configuration {
  pub verbose: bool,
  pub logging: bool
}

impl Configuration {
  pub fn new() -> Self {
    Self{
      verbose: false,
      logging: true
    }
  }
    
}
