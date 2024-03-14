use std::any::Any;

use crate::{application::interfaces::single_action_controller::SingleActionController, domain::services::poems_service::PoemsService};

pub struct TFindPoemById(pub Box<dyn PoemsService>);

impl SingleActionController for TFindPoemById {
    fn execute(&self, data: &dyn Any, handler: Option<&dyn Any>) -> Box<dyn Any> {
        if let Some(id) = data.downcast_ref::<String>() {
            match self.0.read("id", id.as_str()) {
                Ok(poem) => Box::new(poem),
                Err(_) => {
                    panic!("Failed to read poem by id");
                }
            }
        } else if let Some(id) = data.downcast_ref::<&str>() {
            match self.0.read("id", *id) {
                Ok(poem) => Box::new(poem),
                Err(_) => {
                    panic!("Failed to read poem by id");
                }
            }
        } else {
            panic!("Failed to downcast to &str");
        }
    }
}