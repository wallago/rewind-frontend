use dioxus::prelude::*;

use crate::models::Tag;

#[derive(Clone)]
pub struct TagsContext {
    pub tags: Signal<Vec<Tag>>,
    pub refresh: Signal<()>,
}
