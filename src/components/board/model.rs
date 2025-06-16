use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Board {
    pub uuid: String,
    pub name: String,
    pub description: Option<String>,
}
