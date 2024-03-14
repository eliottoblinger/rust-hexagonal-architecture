use std::io;

pub trait ConsoleApp {
    fn run(&self) -> Void;
}

pub struct TConsoleApp();

impl ConsoleApp for TConsoleApp {
    fn run(&self) -> Void{
        
    }
}

#[cfg(test)]
#[path = "../../tests/application/console/adapters/console_app_tests.rs"]
mod console_app_tests;