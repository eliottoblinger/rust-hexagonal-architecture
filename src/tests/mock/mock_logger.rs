use crate::common::logger::Logger;

#[derive(Default, Clone)]
pub struct MockLogger(pub Vec<String>);
impl Logger for MockLogger {
    fn log(&mut self, value: String) {
        self.0.push(value);
    }
}