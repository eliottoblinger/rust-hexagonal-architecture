use std::{fs::{self, File}, io::Read};

use crate::domain::entities::poem::Poem;

pub trait PoemsLoader {
    fn load_poems(&self) -> Vec<Poem>;
}

pub struct TPoemsLoader;

impl PoemsLoader for TPoemsLoader {
    fn load_poems(&self) -> Vec<Poem> {
        let paths = fs::read_dir("./data/poems").unwrap();
        let mut poems: Vec<Poem> = Vec::new();

        for path in paths {
            let path_name = path.unwrap().path();
            let mut file = File::open(path_name).expect("Cannot open file.");

            let mut contents = String::new();
            file.read_to_string(&mut contents).expect("Cannot read file.");

            let result: Result<Poem, serde_json::Error> = serde_json::from_str(&contents);

            match result {
                Ok(poem) => {
                    poems.push(poem);
                },
                Err(e) => {
                    println!("Erreur de désérialisation : {}", e);
                }
            }
        }

        poems
    }
}