use std::io::ErrorKind;

use crate::{domain::entities::poem::Poem, infrastructure::persistence::file_system::{poems_in_file_system::{PoemsInFileSystem, TPoemsInFileSystem}, poems_loader::PoemsLoader}};

fn get_test_poem() -> Poem {
    Poem{
        id: "id2".to_owned(), 
        title: "title2".to_owned(), 
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
        author_id: "author_id2".to_owned()
    }
}

struct MockPoemsLoader();

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

        return vec![poem1, get_test_poem()];
    }
}

#[test]
fn test_get_by_id_with_correct_value() {
    let poems_in_file_system = TPoemsInFileSystem(Box::from(MockPoemsLoader()));

    let poem = poems_in_file_system.get_by_id("id2");

    assert_eq!(poem.unwrap(), get_test_poem());
}

#[test]
fn test_get_by_id_with_wrong_value() {
    let poems_in_file_system = TPoemsInFileSystem(Box::from(MockPoemsLoader()));

    let poem = poems_in_file_system.get_by_id("id4");

    assert!(poem.is_err());
    assert_eq!(poem.unwrap_err().kind(), ErrorKind::NotFound);

}

#[test]
fn test_get_by_title_with_correct_value() {
    let poems_in_file_system = TPoemsInFileSystem(Box::from(MockPoemsLoader()));

    let poem = poems_in_file_system.get_by_title("title2");

    assert_eq!(poem.unwrap(), get_test_poem());
}

#[test]
fn test_get_by_title_with_wrong_value() {
    let poems_in_file_system = TPoemsInFileSystem(Box::from(MockPoemsLoader()));

    let poem = poems_in_file_system.get_by_title("title4");

    assert!(poem.is_err());
    assert_eq!(poem.unwrap_err().kind(), ErrorKind::NotFound);
}

#[test]
fn test_get_by_author_id_with_correct_value() {
    let poems_in_file_system = TPoemsInFileSystem(Box::from(MockPoemsLoader()));

    let poem = poems_in_file_system.get_by_author_id("author_id2");

    assert_eq!(poem.unwrap(), get_test_poem());
}

#[test]
fn test_get_by_author_id_with_wrong_value() {
    let poems_in_file_system = TPoemsInFileSystem(Box::from(MockPoemsLoader()));

    let poem = poems_in_file_system.get_by_author_id("author_id4");

    assert!(poem.is_err());
    assert_eq!(poem.unwrap_err().kind(), ErrorKind::NotFound);
}

#[test]
fn test_get_all() {
    let poems_in_file_system = TPoemsInFileSystem(Box::from(MockPoemsLoader()));

    let poems = poems_in_file_system.get_all();

    assert_eq!(poems, MockPoemsLoader().load_poems());
}