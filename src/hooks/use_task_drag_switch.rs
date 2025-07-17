use dioxus::prelude::*;

use crate::{api::switch_tasks, context::TasksContext};

pub fn use_task_drag_switch(
    mut dragging_from: Signal<Option<String>>,
    mut dragging_to: Signal<Option<String>>,
    mut trigger: Signal<bool>,
) {
    let mut ctx_tasks = use_context::<TasksContext>();

    let mut in_progress = use_signal(|| false);

    let _ = use_resource(move || async move {
        if trigger()
            && let Some(uuid_from) = dragging_from()
            && let Some(uuid_to) = dragging_to()
        {
            in_progress.set(true);
            match switch_tasks(&uuid_from, &uuid_to).await {
                Ok(_) => ctx_tasks.refresh.set(()),
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
