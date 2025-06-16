use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Task {
    pub uuid: String,
    pub name: String,
    pub description: Option<String>,
    pub status: String,
    pub deadline: Option<String>,
}
