use dioxus::prelude::*;

use crate::models::Tag;

#[derive(Clone)]
pub struct TaskTagsContext {
    pub task_tags: Signal<Vec<Tag>>,
    pub refresh: Signal<()>,
}
