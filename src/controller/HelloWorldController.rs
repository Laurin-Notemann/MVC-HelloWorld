use crate::{
    configuration::Configuration::Configuration,
    model::HelloWorldModel::HelloWorldModel, view::HelloWorldView::HelloWorldView,
};

use crate::logger::Logger::Logger;

pub struct HelloWorldController<'a> {
    pub logger: &'a (dyn Logger + 'a),
    pub config: &'a Configuration,
    pub model: &'a mut HelloWorldModel<'a>,
    pub view: &'a HelloWorldView<'a>,
}

impl<'a> HelloWorldController<'a> {
    pub fn new(
        logger: &'a dyn Logger,
        config: &'a Configuration,
        model: &'a mut HelloWorldModel<'a>,
        view: &'a HelloWorldView,
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
