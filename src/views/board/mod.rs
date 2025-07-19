use crate::{
    components::Button,
    context::{ListsContext, TagsContext},
    hooks::{use_lists_get, use_tags_get},
    views::board::{add_list_modal::AddList, header::Header, list_card::ListCard},
};
use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::fa_solid_icons::FaPlus};

mod add_list_modal;
mod add_tag_modal;
mod add_task_modal;
mod delete_list_modal;
mod delete_tag_modal;
mod delete_task_modal;
mod header;
mod list_card;
mod task_card;
mod tasks_card;
mod update_tag_modal;
mod update_tags_modal;
mod update_task_modal;

#[component]
pub fn Board(uuid: String) -> Element {
    use_context_provider(|| ListsContext {
        lists: Signal::new(Vec::new()),
        refresh: Signal::new(()),
    });

    use_lists_get(uuid.clone());

    use_context_provider(|| TagsContext {
        tags: Signal::new(Vec::new()),
        refresh: Signal::new(()),
    });

    use_tags_get(uuid.clone());

    let ctx_lists = use_context::<ListsContext>();

    let mut is_add_open = use_signal(|| false);

    let dragging_index = use_signal(|| None::<String>);
    let lists: Vec<Element> = (ctx_lists.lists)()
        .iter()
        .map(|list| {
            rsx!(
                ListCard { list: list.clone(), dragging_from: dragging_index }
            )
        })
        .collect();

    rsx! {
        div { class: "p-4 h-full bg-primary border-2 border-secondary flex flex-col gap-4",
            div { class: "flex justify-between",
                Header { uuid: uuid.clone() }
                Button {
                    class: "px-2 justify-between gap-2 font-semibold text-base",
                    width: "w-24",
                    onclick: move |_| is_add_open.set(true),
                    "List"
                    Icon { height: 14, width: 14, icon: FaPlus }
                }
            }
            div { class: "overflow-y-auto  h-full grid gap-4 grid-cols-1 sm:grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 2xl:grid-cols-3",
                {lists.iter()}
            }
        }
        AddList { is_open: is_add_open, board_uuid: uuid.clone() }
    }
}
