use dioxus::prelude::*;

use crate::{api::delete_board, context::BoardsContext};

pub fn use_board_delete(uuid: String, mut trigger: Signal<bool>) {
    let mut ctx_boards = use_context::<BoardsContext>();

    let mut in_progress = use_signal(|| false);

    let _ = use_resource(move || {
        let uuid = uuid.clone();
        async move {
            if trigger() {
                in_progress.set(true);
                match delete_board(&uuid).await {
                    Ok(_) => ctx_boards.refresh.set(()),
                    Err(err) => tracing::error!("{err}"),
                };
                in_progress.set(false);
            }
        }
    });

    use_effect(move || {
        if !in_progress() {
            trigger.set(false);
        }
    });
}
