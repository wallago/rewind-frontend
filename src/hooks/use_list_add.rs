use dioxus::prelude::*;

use crate::{api::add_list, context::ListsContext, models::NewList};

pub fn use_list_add(name: Signal<String>, board_uuid: String, mut trigger: Signal<bool>) {
    let mut ctx_lists = use_context::<ListsContext>();

    let mut in_progress = use_signal(|| false);

    let _ = use_resource(move || {
        let board_uuid = board_uuid.clone();
        async move {
            if trigger() {
                in_progress.set(true);
                match add_list(NewList {
                    name: name(),
                    board_uuid,
                    position: None,
                })
                .await
                {
                    Ok(_) => ctx_lists.refresh.set(()),
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
