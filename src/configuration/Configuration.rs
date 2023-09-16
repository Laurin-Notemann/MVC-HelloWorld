pub struct Configuration {
  pub verbose: bool,
  pub logging: bool
}

impl Configuration {
  pub fn new() -> Self {
    Self{
      verbose: false,
      logging: false
    }
  }

  pub fn set_logging(&mut self, logging: bool) {
    self.logging =  logging;
  }
    
}
