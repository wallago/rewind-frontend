use dioxus::prelude::*;

use crate::{api::add_tag, context::TagsContext, models::NewTag};

pub fn use_task_tag_link(task_uuid: String, tag_uuid: String, mut trigger: Signal<bool>) {
    let mut ctx_tags = use_context::<TagsContext>();

    // let mut in_progress = use_signal(|| false);

    // let _ = use_resource(move || {
    //     let board_uuid = board_uuid.clone();
    //     async move {
    //         if trigger() {
    //             in_progress.set(true);
    //             match add_tag(NewTag {
    //                 board_uuid,
    //                 name: name(),
    //                 color: color(),
    //             })
    //             .await
    //             {
    //                 Ok(_) => ctx_tags.refresh.set(()),
    //                 Err(err) => tracing::error!("{err}"),
    //             };
    //             in_progress.set(false);
    //         }
    //     }
    // });

    // use_effect(move || {
    //     if !in_progress() {
    //         trigger.set(false);
    //     }
    // });
}
