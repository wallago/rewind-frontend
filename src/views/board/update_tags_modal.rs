use crate::{
    components::{Button, Dialog, DialogContent, DialogFooter, DialogHeader, DialogTitle, Label},
    context::TagsContext,
    hooks::use_click_outside,
    models::Tag,
    views::board::{delete_tag_modal::DeleteTag, update_tag_modal::UpdateTag},
};
use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::fa_solid_icons::{FaGear, FaXmark},
};

#[component]
pub fn UpdateTags(is_open: Signal<bool>) -> Element {
    let mut is_delete_open = use_signal(|| false);
    let mut is_update_open = use_signal(|| false);
    let mut selected_tag = use_signal(|| None::<Tag>);
    let ctx_tags = use_context::<TagsContext>();

    let tags: Vec<Element> = ctx_tags
        .tags
        .iter()
        .map(|tag| {
            let tag_update = tag.clone();
            let tag_delete = tag.clone();
            rsx! {
                div { class: "flex justify-between",
                    div { class: "flex items-center gap-2",
                        div {
                            style: format!("--tag-color: {};", tag.color),
                            class: "w-4 h-4 bg-[var(--tag-color)] border-2 border-secondary",
                        }
                        Label { variant: "title", "{tag.name}" }
                    }
                    div { class: "flex items-center gap-2",
                        Button {
                            class: "px-1 h-fit py-1 my-1",
                            onclick: move |_| {
                                selected_tag.set(Some(tag_update.clone()));
                                is_update_open.set(true)
                            },
                            Icon { height: 16, width: 16, icon: FaGear }
                        }
                        Button {
                            class: "px-1 h-fit py-1 my-1",
                            onclick: move |_| {
                                selected_tag.set(Some(tag_delete.clone()));
                                is_delete_open.set(true)
                            },
                            Icon { height: 16, width: 16, icon: FaXmark }
                        }
                    }
                }
            }
        })
        .collect();

    use_click_outside(
        "update-tags-area".to_string(),
        move || is_open(),
        EventHandler::new(move |_| {
            if !is_delete_open() && !is_update_open() {
                is_open.set(false)
            }
        }),
    );

    rsx! {
        Dialog { is_open,
            DialogContent { id: "update-tags-area", class: "sm:max-w-[425px]",
                DialogHeader {
                    DialogTitle { "Update Tags" }
                }
                {tags.iter()}
                DialogFooter {
                    Button {
                        onclick: move |_| {
                            is_open.set(false);
                        },
                        r#type: "submit",
                        class: "font-semibold px-2 text-sm",
                        "Close"
                    }
                }
            }
        }
        if let Some(tag) = selected_tag() {
            UpdateTag { tag: tag.clone(), is_open: is_update_open }
            DeleteTag { tag: tag.clone(), is_open: is_delete_open }
        }
    }
}
