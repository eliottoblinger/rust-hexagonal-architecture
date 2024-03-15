use crate::{domain::entities::poem::Poem, infrastructure::persistence::file_system::poems_loader::PoemsLoader};

use super::mock_poem::mock_poem;

#[derive(Clone)]
pub struct MockPoemsLoader();

impl PoemsLoader for MockPoemsLoader {
    fn load_poems(&self) -> Vec<Poem> {
        let poem1 =  Poem{
            id: "id".to_owned(), 
            title: "title".to_owned(), 
            stanzas: vec![
                vec![
                    "stanza1.l1".to_owned(), 
                    "stanza1.l2".to_owned()
                ], 
                vec![
                    "stanza2.l1".to_owned(),
                    "stanza2.l2".to_owned()
                ]
            ],
            author_id: "author_id".to_owned()
        };

        return vec![poem1, mock_poem()];
    }
}