use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct List {
    pub uuid: String,
    pub name: String,
    pub description: Option<String>,
}
