use crate::logger::logger::Logger;
use crate::configuration::configuration::Configuration;
use text_io::read;

pub struct HelloView <'a>{
    pub logger: &'a (dyn Logger + 'a),
    pub config: &'a Configuration,
}

impl<'a> HelloView <'a> {
    pub fn new(logger: &'a dyn Logger, config: &'a Configuration) -> Self{
      Self { logger, config }
    }
    pub fn ask_for_name(&self) -> String {
      println!("Please Enter your Name: ");
      let answer_one: String = read!("{}\n");
      answer_one
    }
    pub fn display_error_message(&self, err_message: &str) {
      self.logger.error(err_message);
    }
    pub fn greet_person(&self, name: String) {
      println!("Hello {}", name)
    }
}
