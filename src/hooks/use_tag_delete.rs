use dioxus::prelude::*;

use crate::{api::delete_tag, context::TagsContext};

pub fn use_tag_delete(uuid: String, mut trigger: Signal<bool>) {
    let mut ctx_tags = use_context::<TagsContext>();

    let mut in_progress = use_signal(|| false);

    let _ = use_resource(move || {
        let uuid = uuid.clone();
        tracing::error!("ldkslfdjks");
        async move {
            if trigger() {
                in_progress.set(true);
                match delete_tag(&uuid).await {
                    Ok(_) => ctx_tags.refresh.set(()),
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
