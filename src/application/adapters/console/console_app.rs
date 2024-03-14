use crate::{application::interfaces::single_action_controller::SingleActionController, domain::{entities::poem::Poem, services::poems_service::PoemsService}};

use super::{actions::{find_poem_by_author_id::TFindPoemByAuthorId, find_poem_by_id::TFindPoemById, find_poem_by_title::TFindPoemByTitle}, types::ArgName};

use serde_json::to_string_pretty;

pub trait ConsoleApp {
    fn run(&self, args: Vec<String>) -> ();
}

pub struct TConsoleApp(pub Box<dyn PoemsService>);

impl ConsoleApp for TConsoleApp {
    fn run(&self, args: Vec<String>) -> () {
        if args.len() != 5 {
            println!("Usage: {} --action <action> <arg_name> <arg>", args[0]);
            return;
        }

        let action = &args[2];
        let arg_name = &args[3];
        let arg = &args[4];
    
        if action != "read" {
            println!("Unknown action: action {} doesn't exists", action);
            return;
        }

        let id_arg_name = ArgName::Id.as_str();
        let title_arg_name = ArgName::Title.as_str();
        let author_id_arg_name = ArgName::AuthorId.as_str();

        if arg_name == author_id_arg_name {
            let find_poem_by_author_id = TFindPoemByAuthorId(self.0.clone());
            find_poem_by_author_id.execute(arg, None);
            return;
        }

        if arg_name == title_arg_name {
            let find_poem_by_title = TFindPoemByTitle(self.0.clone());
            find_poem_by_title.execute(arg, None);
            return;
        }

        if arg_name == id_arg_name {
            let find_poem_by_id = TFindPoemById(self.0.clone());
            let poem = find_poem_by_id.execute(arg, None) as Box<Poem>;

            let serialized_poem = to_string_pretty(&poem).unwrap();

            println!("{}", serialized_poem);
            return;
        }
    }
}

#[cfg(test)]
#[path = "../../../tests/application/adapters/console/console_app_tests.rs"]
mod console_app_tests;