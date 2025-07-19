use crate::{
    components::{
        Button, Dialog, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader,
        DialogTitle,
    },
    hooks::{use_click_outside, use_tag_delete},
    models::Tag,
};
use dioxus::prelude::*;

#[component]
pub fn DeleteTag(tag: Tag, is_open: Signal<bool>) -> Element {
    let mut trigger = use_signal(|| false);

    use_tag_delete(tag.uuid, trigger);

    use_click_outside(
        "delete-tag-area".to_string(),
        move || is_open(),
        EventHandler::new(move |_| is_open.set(false)),
    );

    rsx! {
        Dialog { is_open,
            DialogContent { id: "delete-tag-area", class: "sm:max-w-[425px]",
                DialogHeader {
                    DialogTitle { "Delete {tag.name}" }
                    DialogDescription { "Are you sure ?" }
                }
                DialogFooter {
                    DialogClose {}
                    Button {
                        onclick: move |_| {
                            trigger.set(true);
                            is_open.set(false);
                        },
                        r#type: "submit",
                        variant: "outline",
                        class: "font-semibold px-2 text-sm",
                        "Delete"
                    }
                }
            }
        }
    }
}
