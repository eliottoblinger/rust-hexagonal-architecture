use std::any::Any;

use crate::{application::interfaces::single_action_controller::SingleActionController, domain::services::poems_service::PoemsService};

pub struct TFindPoemByAuthorId(pub Box<dyn PoemsService>);

impl SingleActionController for TFindPoemByAuthorId {
    fn execute(&self, data: &dyn Any, _handler: Option<&dyn Any>) -> Box<dyn Any> {
        if let Some(author_id) = data.downcast_ref::<String>() {
            match self.0.read("author_id", author_id.as_str()) {
                Ok(poem) => Box::new(poem),
                Err(_) => {
                    panic!("Failed to read poem by author id");
                }
            }
        } else if let Some(author_id) = data.downcast_ref::<&str>() {
            match self.0.read("author_id", *author_id) {
                Ok(poem) => Box::new(poem),
                Err(_) => {
                    panic!("Failed to read poem by author id");
                }
            }
        } else {
            panic!("Failed to downcast to &str");
        }
    }
}