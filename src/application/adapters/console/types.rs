use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum ArgName {
    Id,
    Title,
    AuthorId
}

impl FromStr for ArgName {
    type Err = ();

    fn from_str(input: &str) -> Result<ArgName, Self::Err> {
        match input {
            "id"  => Ok(ArgName::Id),
            "title"  => Ok(ArgName::Title),
            "author_id"  => Ok(ArgName::AuthorId),
            _ => Err(()),
        }
    }
}