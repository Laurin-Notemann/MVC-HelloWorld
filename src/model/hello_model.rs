use crate::{configuration::configuration::Configuration, logger::logger::Logger};

pub struct HelloModel <'a>  {
    pub logger: &'a (dyn Logger + 'a),
    pub config: &'a Configuration,
    pub name: Option<String>,
}

impl<'a> HelloModel <'a>{
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
