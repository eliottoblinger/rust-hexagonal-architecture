use std::io;

use crate::domain::entities::poem::Poem;
use crate::infrastructure::persistence::file_system::poems_in_file_system::PoemsInFileSystem; 

pub trait PoemsService {
    fn get_poems(&self) -> Vec<Poem>;
    fn read(&self, field: &str, value: &str) -> Result<Poem, io::Error>;
}

pub struct TPoemsService(pub Box<dyn PoemsInFileSystem>);

impl PoemsService for TPoemsService {
    fn get_poems(&self) -> Vec<Poem> {
        return self.0.get_all();
    }
       
    fn read(&self, field: &str, value: &str) -> Result<Poem, io::Error> {
        let poem = match field {
            "id" => self.0.get_by_id(value),
            "title" => self.0.get_by_title(value),
            "author_id" => self.0.get_by_author_id(value),
            _ => Err(io::Error::new(io::ErrorKind::NotFound, "Poème non trouvé")),
        };
    
       return poem;
    }
}

#[cfg(test)]
#[path = "../../tests/domain/services/poems_service_tests.rs"]
mod poems_service_tests;