use dioxus::prelude::*;

use crate::{api::get_tags_by_board_uuid, context::TagsContext};

pub fn use_tags_get(board_uuid: String) {
    let ctx_tags = use_context::<TagsContext>();
    let _ = use_resource({
        let mut tags = ctx_tags.tags.clone();
        let refresh = ctx_tags.refresh.clone();
        let board_uuid = board_uuid.clone();
        move || {
            let board_uuid = board_uuid.clone();
            async move {
                refresh();
                match get_tags_by_board_uuid(&board_uuid).await {
                    Ok(fetched) => tags.set(fetched),
                    Err(err) => tracing::error!("{err}"),
                }
            }
        }
    });
}
