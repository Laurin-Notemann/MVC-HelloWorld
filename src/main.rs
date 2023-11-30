use configuration::configuration::Configuration;
use controller::hello_controller::HelloController;
use logger::logger::ScopedLogger;
use model::hello_model::HelloModel;
use view::hello_wiew::HelloView;
mod configuration;
mod controller;
mod logger;
mod model;
mod view;

fn main() {
    let prefixes = vec!["Hello-Program".to_owned()];
    let hello_logger = ScopedLogger::new(prefixes);
    let config = Configuration::new();

    let mut model = HelloModel::new(&hello_logger, &config);
    let view = HelloView::new(&hello_logger, &config);

    let mut hello = HelloController::new(&hello_logger, &config, &mut model, &view);

    hello.ask_and_save_name();
}
