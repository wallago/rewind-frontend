use dioxus::prelude::*;

use crate::{api::get_tasks_by_list_uuid, context::TasksContext};

pub fn use_tasks_get(list_uuid: String) {
    let ctx_tasks = use_context::<TasksContext>();
    let _ = use_resource({
        let mut tasks = ctx_tasks.tasks.clone();
        let refresh = ctx_tasks.refresh.clone();
        let list_uuid = list_uuid.clone();
        move || {
            let list_uuid = list_uuid.clone();
            async move {
                refresh();
                match get_tasks_by_list_uuid(list_uuid).await {
                    Ok(fetched) => tasks.set(fetched),
                    Err(err) => tracing::error!("{err}"),
                }
            }
        }
    });
}
