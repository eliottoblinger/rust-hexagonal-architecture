use crate::domain::entities::poem::Poem;

pub fn mock_poem() -> Poem {
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