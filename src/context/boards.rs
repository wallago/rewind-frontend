use dioxus::prelude::*;

use crate::models::Board;

#[derive(Clone)]
pub struct BoardsContext {
    pub boards: Signal<Vec<Board>>,
    pub refresh: Signal<()>,
}
