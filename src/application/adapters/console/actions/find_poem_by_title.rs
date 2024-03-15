use std::any::Any;

use crate::{application::interfaces::single_action_controller::SingleActionController, domain::services::poems_service::PoemsService};

pub struct TFindPoemByTitle(pub Box<dyn PoemsService>);

impl SingleActionController for TFindPoemByTitle {
    fn execute(&self, data: &dyn Any, _handler: Option<&dyn Any>) -> Box<dyn Any> {
        if let Some(title) = data.downcast_ref::<String>() {
            match self.0.read("title", title.as_str()) {
                Ok(poem) => Box::new(poem),
                Err(_) => {
                    panic!("Failed to read poem by title");
                }
            }
        } else if let Some(title) = data.downcast_ref::<&str>() {
            match self.0.read("title", *title) {
                Ok(poem) => Box::new(poem),
                Err(_) => {
                    panic!("Failed to read poem by title");
                }
            }
        } else {
            panic!("Failed to downcast to &str");
        }
    }
}