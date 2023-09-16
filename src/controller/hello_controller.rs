use crate::{
    configuration::configuration::Configuration,
    model::hello_model::HelloModel, view::hello_wiew::HelloView,
};

use crate::logger::logger::Logger;

pub struct HelloController<'a> {
    pub logger: &'a (dyn Logger + 'a),
    pub config: &'a Configuration,
    pub model: &'a mut HelloModel<'a>,
    pub view: &'a HelloView<'a>,
}

impl<'a> HelloController<'a> {
    pub fn new(
        logger: &'a dyn Logger,
        config: &'a Configuration,
        model: &'a mut HelloModel<'a>,
        view: &'a HelloView,
    ) -> Self {
        Self {
            logger,
            config,
            model,
            view,
        }
    }

    pub fn ask_and_save_name(&mut self) {
        self.logger.info("dangs");
        self.save_name(self.ask_name());
        self.output_name();
    }

    fn ask_name(&self) -> String {
        return self.view.ask_for_name();
    }

    fn save_name(&mut self, name: String) {
        self.model.save_name(name);
    }

    fn output_name(&self) {
        match self.model.get_name() {
            Some(name) => self.view.greet_person(name),
            None => self.view.display_error_message("There is no name saved"),
        };
        
    }
}
