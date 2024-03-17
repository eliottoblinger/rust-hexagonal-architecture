use crate::{application::adapters::console::console_app::{ConsoleApp, TConsoleApp}, domain::services::poems_service::TPoemsService, infrastructure::persistence::file_system::poems_in_file_system::TPoemsInFileSystem, tests::mock::{mock_logger::MockLogger, mock_poem::mock_poem, mock_poems_repository::MockPoemsRepository}};

#[test]
fn test_run_with_id() {
    let poems_service = TPoemsService(Box::from(TPoemsInFileSystem(Box::from(MockPoemsRepository()))));
    let mut mock_logger = MockLogger::default();

    let mut console_app = TConsoleApp { poems_service: Box::from(poems_service), logger: &mut mock_logger };

    console_app.run(vec!["".to_owned(), "--action".to_owned(), "read".to_owned(), "id".to_owned(), "id2".to_owned()]);

    println!("{}", mock_logger.0[0]);

    assert_eq!(format!("{:?}", mock_poem()) , mock_logger.0[0]);
}
