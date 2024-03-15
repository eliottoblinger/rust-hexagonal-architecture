use dyn_clone::DynClone;

pub trait Logger: DynClone {
    fn log(&mut self, value: String);
}

dyn_clone::clone_trait_object!(Logger);

#[derive(Clone)]
pub struct ConsoleLogger;

impl Logger for ConsoleLogger {
    fn log(&mut self, value: String) {
        println!("{}", value);
    }
}