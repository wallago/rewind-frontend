use crate::{
    context::BoardsContext,
    views::home::{board_card::BoardCard, header::Header},
};
use dioxus::prelude::*;

mod board_card;
mod delete_board_modal;
mod header;

#[component]
pub fn Home() -> Element {
    let ctx_boards = use_context::<BoardsContext>();
    let dragging_index = use_signal(|| None::<String>);
    let boards: Vec<Element> = (ctx_boards.boards)()
        .iter()
        .map(|board| {
            rsx!(BoardCard {
                board: board.clone(),
                dragging_from: dragging_index
            })
        })
        .collect();

    rsx! {
        div { class: "p-4 h-full bg-primary border-2 border-secondary flex flex-col gap-4",
            Header {}
            div { class: "grid gap-4 grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5 overflow-y-auto",
                {boards.iter()}
            }
        }
    }
}
