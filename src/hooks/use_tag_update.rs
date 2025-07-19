use dioxus::prelude::*;

use crate::{api::update_tag, context::TagsContext, models::UpdateTag};

pub fn use_tag_update(
    name: Signal<String>,
    color: Signal<String>,
    uuid: String,
    mut trigger: Signal<bool>,
) {
    let mut ctx_tags = use_context::<TagsContext>();

    let mut in_progress = use_signal(|| false);

    let _ = use_resource(move || {
        let uuid = uuid.clone();
        async move {
            if trigger() {
                in_progress.set(true);
                match update_tag(
                    &uuid,
                    UpdateTag {
                        name: Some(name()),
                        color: Some(color()),
                    },
                )
                .await
                {
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
