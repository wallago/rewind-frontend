use dioxus::prelude::*;

use crate::models::List;

#[derive(Clone)]
pub struct ListsContext {
    pub lists: Signal<Vec<List>>,
    pub refresh: Signal<()>,
}
