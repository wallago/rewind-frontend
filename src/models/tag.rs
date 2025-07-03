use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Deserialize, Serialize)]
pub struct Tag {
    pub name: String,
    pub color: String,
}
