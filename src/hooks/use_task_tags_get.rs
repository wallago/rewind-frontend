use dioxus::prelude::*;

use crate::{api::get_tags_by_task_uuid, context::TaskTagsContext};

pub fn use_task_tags_get(task_uuid: String) {
    let ctx_task_tags = use_context::<TaskTagsContext>();
    let _ = use_resource({
        let mut task_tags = ctx_task_tags.task_tags.clone();
        let refresh = ctx_task_tags.refresh.clone();
        let task_uuid = task_uuid.clone();
        move || {
            let task_uuid = task_uuid.clone();
            async move {
                refresh();
                match get_tags_by_task_uuid(&task_uuid).await {
                    Ok(fetched) => task_tags.set(fetched),
                    Err(err) => tracing::error!("{err}"),
                }
            }
        }
    });
}
