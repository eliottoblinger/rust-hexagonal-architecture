use application::adapters::console::console_app::{ConsoleApp, TConsoleApp};
use common::logger::ConsoleLogger;
use domain::services::poems_service::TPoemsService;
use infrastructure::persistence::file_system::{poems_in_file_system::TPoemsInFileSystem, poems_loader::TPoemsLoader};

mod infrastructure;
mod domain;
mod application;
mod common;
mod tests;

fn main() {
    let poems_service = TPoemsService(Box::from(TPoemsInFileSystem(Box::from(TPoemsLoader))));
    let mut logger = ConsoleLogger {};
    let mut console_app = TConsoleApp { poems_service: Box::from(poems_service), logger: &mut logger };

    let args: Vec<String> = std::env::args().collect();

    console_app.run(args);
 }
