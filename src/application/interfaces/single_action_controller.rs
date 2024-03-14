use std::any::Any;

pub trait SingleActionController {
    fn execute(&self, data: &dyn Any, handler: Option<&dyn Any>) -> Box<dyn Any>;
}