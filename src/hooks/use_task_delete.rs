use dioxus::prelude::*;

use crate::{api::delete_task, context::TasksContext};

pub fn use_task_delete(uuid: String, mut trigger: Signal<bool>) {
    let mut ctx_tasks = use_context::<TasksContext>();

    let mut in_progress = use_signal(|| false);

    let _ = use_resource(move || {
        let uuid = uuid.clone();
        async move {
            if trigger() {
                in_progress.set(true);
                match delete_task(&uuid).await {
                    Ok(_) => ctx_tasks.refresh.set(()),
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
