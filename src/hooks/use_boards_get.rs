use dioxus::prelude::*;

use crate::{api::get_boards, context::BoardsContext};

pub fn use_boards_get() {
    let ctx_boards = use_context::<BoardsContext>();

    let _ = use_resource({
        let mut boards = ctx_boards.boards.clone();
        let refresh = ctx_boards.refresh.clone();
        move || async move {
            refresh();
            match get_boards().await {
                Ok(fetched) => boards.set(fetched),
                Err(err) => tracing::error!("{err}"),
            }
        }
    });
}
