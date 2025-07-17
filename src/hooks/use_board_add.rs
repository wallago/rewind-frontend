use dioxus::prelude::*;

use crate::{api::add_board, context::BoardsContext, models::NewBoard};

pub fn use_board_add(name: Signal<String>, mut trigger: Signal<bool>) {
    let mut ctx_boards = use_context::<BoardsContext>();

    let mut in_progress = use_signal(|| false);

    let _ = use_resource(move || async move {
        if trigger() {
            in_progress.set(true);
            match add_board(NewBoard {
                name: name(),
                position: None,
            })
            .await
            {
                Ok(_) => ctx_boards.refresh.set(()),
                Err(err) => tracing::error!("{err}"),
            };
            in_progress.set(false);
        }
    });

    use_effect(move || {
        if !in_progress() {
            trigger.set(false);
        }
    });
}
