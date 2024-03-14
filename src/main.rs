use application::adapters::console::console_app::{ConsoleApp, TConsoleApp};
use domain::services::poems_service::TPoemsService;
use infrastructure::persistence::file_system::{poems_in_file_system::TPoemsInFileSystem, poems_loader::TPoemsLoader};

mod infrastructure;
mod domain;
mod application;

fn main() {
    let poems_service = TPoemsService(Box::from(TPoemsInFileSystem(Box::from(TPoemsLoader))));
    let console_app = TConsoleApp(Box::from(poems_service));

    let args: Vec<String> = std::env::args().collect();

    console_app.run(args);
 }
