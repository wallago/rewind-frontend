use dioxus::prelude::*;

use crate::{api::unlink_task_tag, context::TaskTagsContext, models::TaskTag};

pub fn use_task_tag_unlink(task_uuid: String, tag_uuid: String, mut trigger: Signal<bool>) {
    let mut ctx_task_tags = use_context::<TaskTagsContext>();

    let mut in_progress = use_signal(|| false);

    let _ = use_resource(move || {
        let task_uuid = task_uuid.clone();
        let tag_uuid = tag_uuid.clone();
        async move {
            if trigger() {
                in_progress.set(true);
                match unlink_task_tag(TaskTag {
                    task_uuid,
                    tag_uuid,
                })
                .await
                {
                    Ok(_) => ctx_task_tags.refresh.set(()),
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
