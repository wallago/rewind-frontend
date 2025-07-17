use dioxus::prelude::*;

use crate::{api::switch_lists, context::ListsContext};

pub fn use_list_drag_switch(
    mut dragging_from: Signal<Option<String>>,
    mut dragging_to: Signal<Option<String>>,
    mut trigger: Signal<bool>,
) {
    let mut ctx_lists = use_context::<ListsContext>();

    let mut in_progress = use_signal(|| false);

    let _ = use_resource(move || async move {
        if trigger()
            && let Some(uuid_from) = dragging_from()
            && let Some(uuid_to) = dragging_to()
        {
            in_progress.set(true);
            match switch_lists(&uuid_from, &uuid_to).await {
                Ok(_) => ctx_lists.refresh.set(()),
                Err(err) => tracing::error!("{err}"),
            };
            in_progress.set(false);
        }
    });

    use_effect(move || {
        if !in_progress() {
            dragging_from.set(None);
            dragging_to.set(None);
            trigger.set(false);
        }
    });
}
