use std::io;

use dyn_clone::DynClone;

use crate::{domain::entities::poem::Poem, repositories::poems_repository::PoemsRepository};
pub trait PoemsInFileSystem: DynClone {
    fn get_all(&self) -> Vec<Poem>;
    fn get_by_id(&self, id: &str) -> Result<Poem, io::Error>;
    fn get_by_title(&self, title: &str) -> Result<Poem, io::Error>;
    fn get_by_author_id(&self, author_id: &str) -> Result<Poem, io::Error>;
}

dyn_clone::clone_trait_object!(PoemsInFileSystem);

#[derive(Clone)]
pub struct TPoemsInFileSystem(pub Box<dyn PoemsRepository>);

impl PoemsInFileSystem for TPoemsInFileSystem {
    fn get_all(&self) -> Vec<Poem> {
        return self.0.load_poems();
    }

    fn get_by_id(&self, id: &str) -> Result<Poem, io::Error> {
        let poems = self.get_all();

        let poem = poems.iter().find(|poem| poem.id == id);
    
        match poem {
            Some(poem) => Ok(poem.clone()),
            None => Err(io::Error::new(io::ErrorKind::NotFound, "Poème non trouvé")),
        }
    }

    fn get_by_title(&self, title: &str) -> Result<Poem, io::Error> {
        let poems = self.get_all();

        let poem = poems.iter().find(|poem| poem.title == title);
    
        match poem {
            Some(poem) => Ok(poem.clone()),
            None => Err(io::Error::new(io::ErrorKind::NotFound, "Poème non trouvé")),
        }
    }

    fn get_by_author_id(&self, author_id: &str) -> Result<Poem, io::Error> {
        let poems = self.get_all();

        let poem = poems.iter().find(|poem| poem.author_id == author_id);
    
        match poem {
            Some(poem) => Ok(poem.clone()),
            None => Err(io::Error::new(io::ErrorKind::NotFound, "Poème non trouvé")),
        }
    }
}

#[cfg(test)]
#[path = "../../../tests/infrastructure/persistence/file_system/poems_in_file_system_tests.rs"]
mod poems_in_file_system_tests;