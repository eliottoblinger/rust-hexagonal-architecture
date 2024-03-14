pub enum ArgName {
    Id,
    Title,
    AuthorId
}

impl ArgName {
    pub fn as_str(&self) -> &'static str {
        match self {
            ArgName::Id => "id",
            ArgName::Title => "title",
            ArgName::AuthorId => "author_id"
        }
    }
}