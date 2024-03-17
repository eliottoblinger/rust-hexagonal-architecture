use std::io::ErrorKind;

use crate::{domain::services::poems_service::TPoemsService, infrastructure::persistence::file_system::poems_in_file_system::TPoemsInFileSystem, repositories::poems_repository::PoemsRepository, tests::mock::{mock_poem::mock_poem, mock_poems_repository::MockPoemsRepository}};
use crate::domain::services::poems_service::PoemsService;

#[test]
fn test_get_by_id_with_correct_value() {
    let poems_service = TPoemsService(Box::from(TPoemsInFileSystem(Box::from(MockPoemsRepository()))));

    let poem = poems_service.read("id", "id2");

    assert_eq!(poem.unwrap(), mock_poem());
}

#[test]
fn test_get_by_id_with_wrong_value() {
    let poems_service = TPoemsService(Box::from(TPoemsInFileSystem(Box::from(MockPoemsRepository()))));

    let poem = poems_service.read("id", "id4");

    assert!(poem.is_err());
    assert_eq!(poem.unwrap_err().kind(), ErrorKind::NotFound);
}

#[test]
fn test_get_by_title_with_correct_value() {
    let poems_service = TPoemsService(Box::from(TPoemsInFileSystem(Box::from(MockPoemsRepository()))));

    let poem = poems_service.read("title", "title2");

    assert_eq!(poem.unwrap(), mock_poem());
}

#[test]
fn test_get_by_title_with_wrong_value() {
    let poems_service = TPoemsService(Box::from(TPoemsInFileSystem(Box::from(MockPoemsRepository()))));

    let poem = poems_service.read("title", "title4");

    assert!(poem.is_err());
    assert_eq!(poem.unwrap_err().kind(), ErrorKind::NotFound);
}

#[test]
fn test_get_by_author_id_with_correct_value() {
    let poems_service = TPoemsService(Box::from(TPoemsInFileSystem(Box::from(MockPoemsRepository()))));

    let poem = poems_service.read("author_id", "author_id2");

    assert_eq!(poem.unwrap(), mock_poem());
}

#[test]
fn test_get_by_author_id_with_wrong_value() {
   
 let poems_service = TPoemsService(Box::from(TPoemsInFileSystem(Box::from(MockPoemsRepository()))));
    let poem = poems_service.read("author_id", "author_id4");

    assert!(poem.is_err());
    assert_eq!(poem.unwrap_err().kind(), ErrorKind::NotFound);
}

#[test]
fn test_get_poems() {
    let poems_service = TPoemsService(Box::from(TPoemsInFileSystem(Box::from(MockPoemsRepository()))));

    let poems = poems_service.get_poems();

    assert_eq!(poems, MockPoemsRepository().load_poems());
}