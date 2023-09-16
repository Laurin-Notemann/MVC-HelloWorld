use configuration::Configuration::Configuration;
use controller::HelloWorldController::HelloWorldController;
use logger::Logger::{ConsLog, ScopedLogger};
use model::HelloWorldModel::HelloWorldModel;
use view::HelloWorldView::HelloWorldView;

mod model;
mod view;
mod controller;
mod logger;
mod configuration;

fn main() {
  let logger = ConsLog::new();

  let prefixes = vec!["Hello".to_owned()];
  let hello_logger = ScopedLogger::new(prefixes);
  let config = Configuration::new();

  let mut model = HelloWorldModel::new(&hello_logger, &config);
  let view = HelloWorldView::new(&logger, &config);

  let mut hello = HelloWorldController::new(&logger, &config, &mut model, &view);

  hello.ask_and_save_name();

}
