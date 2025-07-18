use serde::{Deserialize, Serialize};

use crate::models::{Priority, Status};

#[derive(Serialize)]
pub struct TaskTag {
    pub task_uuid: String,
    pub tag_uuid: String,
}
