use crate::domain::entities::poem::Poem;

#[derive(Clone)]
struct MockPoemsLoader();

#[test]
fn test_new_poem_has_all_fields() {
    let id = "id2".to_owned();
    let title =  "title2".to_owned();
    let stanzas = vec![
        vec![
            "stanza1.l1".to_owned(), 
            "stanza1.l2".to_owned()
        ], 
        vec![
            "stanza2.l1".to_owned(),
            "stanza2.l2".to_owned()
        ]
    ];
    let author_id = "author_id2".to_owned();

    let poem =  Poem{
        id: id.clone(),
        title: title.clone(),
        stanzas: stanzas.clone(),
        author_id: author_id.clone(),
    };

    assert_eq!(poem.id, id);
    assert_eq!(poem.title, title);
    assert_eq!(poem.stanzas, stanzas);
    assert_eq!(poem.author_id, author_id);
}