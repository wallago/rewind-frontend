use dioxus::prelude::*;

use crate::{api::update_board, context::BoardsContext, models::UpdateBoard};

pub fn use_board_update(name: Signal<String>, uuid: String, mut trigger: Signal<bool>) {
    let mut ctx_boards = use_context::<BoardsContext>();

    let mut in_progress = use_signal(|| false);

    let _ = use_resource(move || {
        let uuid = uuid.clone();
        async move {
            if trigger() {
                in_progress.set(true);
                match update_board(
                    &uuid,
                    UpdateBoard {
                        name: Some(name()),
                        position: None,
                    },
                )
                .await
                {
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
