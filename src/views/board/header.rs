use crate::{
    Route,
    api::get_board_by_uuid,
    components::{Button, Label},
};
use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::fa_solid_icons::FaChevronRight};

#[component]
pub fn Header(uuid: String) -> Element {
    let board = use_resource(move || {
        let uuid = uuid.clone();
        async move {
            match get_board_by_uuid(&uuid.clone()).await {
                Ok(fetched) => Some(fetched),
                Err(err) => {
                    tracing::error!("{err}");
                    None
                }
            }
        }
    });

    rsx! {
        div { class: "flex gap-4 items-center",
            Button {
                class: "px-2 text-base",
                onclick: move |_| {
                    navigator().push(Route::Home {});
                },
                "Boards"
            }
            Icon { class: "text-secondary", height: 20, icon: FaChevronRight }
            {
                match board() {
                    Some(Some(board)) => rsx! {
                        Label { class: "w-fit px-2 truncate text-base py-1.5", {board.name} }
                    },
                    _ => rsx! {
                        Label { class: "w-fit px-2 truncate text-base py-1.5", "Loading" }
                    },
                }
            }
        }
    }
}
