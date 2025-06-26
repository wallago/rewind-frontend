use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Tag {
    pub uuid: String,
    pub name: String,
    pub color: String,
}

#[derive(Serialize, Clone)]
pub struct NewTag {
    pub name: String,
    pub color: String,
}

#[derive(Serialize, Clone)]
pub struct UpdateTag {
    pub name: Option<String>,
    pub color: Option<String>,
}
