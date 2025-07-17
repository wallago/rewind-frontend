use dioxus::prelude::*;

use crate::{api::delete_list, context::ListsContext};

pub fn use_list_delete(uuid: String, mut trigger: Signal<bool>) {
    let mut ctx_lists = use_context::<ListsContext>();

    let mut in_progress = use_signal(|| false);

    let _ = use_resource(move || {
        let uuid = uuid.clone();
        async move {
            if trigger() {
                in_progress.set(true);
                match delete_list(&uuid).await {
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
