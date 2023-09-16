use crate::{configuration::Configuration::Configuration, logger::Logger::Logger};

pub struct HelloWorldModel <'a>  {
    pub logger: &'a (dyn Logger + 'a),
    pub config: &'a Configuration,
    pub name: Option<String>,
}

impl<'a> HelloWorldModel <'a>{
    pub fn new(logger: &'a dyn Logger, config: &'a Configuration) -> Self {
        if config.logging {
          logger.info("")
        }
        Self {
            logger,
            config,
            name: None,
        }
    }
    pub fn save_name(&mut self, name: String) {
      self.name = Some(name)
    }
    pub fn get_name(&self) -> Option<String>{
      self.name.clone()
    }
}
