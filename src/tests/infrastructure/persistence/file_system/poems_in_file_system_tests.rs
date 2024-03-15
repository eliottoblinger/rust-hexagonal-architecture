use std::io::ErrorKind;

use crate::{infrastructure::persistence::file_system::{poems_in_file_system::{PoemsInFileSystem, TPoemsInFileSystem}, poems_loader::PoemsLoader}, tests::mock::{mock_poem::mock_poem, mock_poems_loader::MockPoemsLoader}};

#[test]
fn test_get_by_id_with_correct_value() {
    let poems_in_file_system = TPoemsInFileSystem(Box::from(MockPoemsLoader()));

    let poem = poems_in_file_system.get_by_id("id2");

    assert_eq!(poem.unwrap(), mock_poem());
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

    assert_eq!(poem.unwrap(), mock_poem());
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

    assert_eq!(poem.unwrap(), mock_poem());
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