use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct Poem {
    pub id: String,
    pub title: String,
    pub stanzas: Vec<Vec<String>>,
    pub author_id: String
}