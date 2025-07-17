use dioxus::prelude::*;

use crate::{api::get_lists_by_board_uuid, context::ListsContext};

pub fn use_lists_get(board_uuid: String) {
    let ctx_lists = use_context::<ListsContext>();
    let _ = use_resource({
        let mut lists = ctx_lists.lists.clone();
        let refresh = ctx_lists.refresh.clone();
        let board_uuid = board_uuid.clone();
        move || {
            let board_uuid = board_uuid.clone();
            async move {
                refresh();
                match get_lists_by_board_uuid(board_uuid).await {
                    Ok(fetched) => lists.set(fetched),
                    Err(err) => tracing::error!("{err}"),
                }
            }
        }
    });
}
