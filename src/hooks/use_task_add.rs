use dioxus::prelude::*;

use crate::{api::add_task, context::TasksContext, models::NewTask};

pub fn use_task_add(name: Signal<String>, list_uuid: String, mut trigger: Signal<bool>) {
    let mut ctx_tasks = use_context::<TasksContext>();

    let mut in_progress = use_signal(|| false);

    let _ = use_resource(move || {
        let list_uuid = list_uuid.clone();
        async move {
            if trigger() {
                in_progress.set(true);
                match add_task(NewTask {
                    name: name(),
                    list_uuid,
                    position: None,
                    priority: None,
                    status: None,
                })
                .await
                {
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
