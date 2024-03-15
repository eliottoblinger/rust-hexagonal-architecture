use std::{any::Any, str::FromStr};

use crate::{application::interfaces::single_action_controller::SingleActionController, common::logger::Logger, domain::{entities::poem::Poem, services::poems_service::PoemsService}};

use super::{actions::{find_poem_by_author_id::TFindPoemByAuthorId, find_poem_by_id::TFindPoemById, find_poem_by_title::TFindPoemByTitle}, types::ArgName};

pub trait ConsoleApp {
    fn run(&mut self, args: Vec<String>) -> ();
}

pub struct TConsoleApp<'a> {
    pub poems_service: Box<dyn PoemsService>,
    pub logger: &'a mut dyn Logger,
}

impl ConsoleApp for TConsoleApp<'_> {
    fn run(&mut self, args: Vec<String>) -> () {
        if args.len() != 5 {
            self.logger.log(format!("Usage: {} --action <action> <arg_name> <arg>", args[0]));   
            return;
        }

        let input_action = &args[2];
        let input_arg_name = &args[3];
        let input_arg = &args[4];
    
        if input_action != "read" {
            self.logger.log(format!("Unknown action: action {} doesn't exists", input_action));
            return;
        }

        let arg_name = ArgName::from_str(&input_arg_name);

        match arg_name {
            Ok(ArgName::Id)  => handle_poem_execution(TFindPoemById(self.poems_service.clone()).execute(input_arg, None), &input_action, &input_arg_name, &input_arg, self.logger),
            Ok(ArgName::Title)  => handle_poem_execution(TFindPoemByTitle(self.poems_service.clone()).execute(input_arg, None), &input_action, &input_arg_name, &input_arg, self.logger),
            Ok(ArgName::AuthorId ) => handle_poem_execution(TFindPoemByAuthorId(self.poems_service.clone()).execute(input_arg, None), &input_action, &input_arg_name, &input_arg, self.logger),
            Err(_) => self.logger.log(format!("Unknown arg_name: arg_name {} doesn't exists", input_arg_name)),
        }
    }
}

fn handle_poem_execution(poem: Box<dyn Any>, input_action: &str, input_arg_name: &str, input_arg: &str, logger: &mut dyn Logger){
    if let Ok(poem) = poem.downcast::<Poem>() {
        logger.log(format!("{:?}", *poem));
        return;
    }

    logger.log(format!("Error while executing action {} with arg_name {} and arg {}", input_action, input_arg_name, input_arg));
}

#[cfg(test)]
#[path = "../../../tests/application/adapters/console/console_app_tests.rs"]
mod console_app_tests;