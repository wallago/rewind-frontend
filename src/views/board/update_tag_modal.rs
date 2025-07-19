use crate::{
    components::{
        Button, Dialog, DialogClose, DialogContent, DialogFooter, DialogHeader, DialogTitle, Input,
        Label,
    },
    hooks::{use_click_outside, use_tag_update},
    models::Tag,
};
use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::fa_solid_icons::FaXmark};
use regex::Regex;

#[component]
pub fn UpdateTag(tag: Tag, is_open: Signal<bool>) -> Element {
    let re = Regex::new(r"^#[0-9a-fA-F]{6}$")?;
    let name = use_signal(|| tag.name.clone());
    let color = use_signal(|| tag.color.clone());
    let mut is_valid_hex = use_signal(|| false);
    let mut trigger = use_signal(|| false);

    use_tag_update(name, color, tag.uuid.clone(), trigger);

    use_click_outside(
        "update-tag-area".to_string(),
        move || is_open(),
        EventHandler::new(move |_| is_open.set(false)),
    );

    use_effect(move || {
        if !re.is_match(&color().trim()) {
            is_valid_hex.set(false)
        } else {
            is_valid_hex.set(true)
        }
    });

    rsx! {
        Dialog { is_open,
            DialogContent { id: "update-tag-area", class: "sm:max-w-[425px]",
                DialogHeader {
                    DialogTitle { "Update {tag.name}" }
                }
                Input {
                    label: "name:\u{00A0}",
                    width: "w-full",
                    placeholder: "Enter tag name",
                    value: name,
                    onenter: EventHandler::new(move |_e: KeyboardEvent| {
                        trigger.set(true);
                        is_open.set(false);
                    }),
                }
                Input {
                    label: "color:",
                    width: "w-full",
                    placeholder: "Enter tag color",
                    value: color,
                }
                div {
                    class: "flex gap-2 items-center pt-1",
                    Label { variant: "title", "preview color:" }
                    if is_valid_hex() {
                        div {
                            style:  format!("--tag-color: {};", color),
                            class: "w-6 h-6 bg-[var(--tag-color)] border-2 border-secondary",
                        }
                    } else {
                        div {
                            class: "w-6 h-6 bg-primary border-2 border-secondary",
                            Icon { height: 20, width: 20, icon: FaXmark }
                        }
                    }
                }
                DialogFooter {
                    DialogClose {}
                    Button {
                        disabled: !is_valid_hex(),
                        onclick: move |_| {
                            trigger.set(true);
                            is_open.set(false);
                        },
                        r#type: "submit",
                        variant: "outline",
                        class: "font-semibold px-2 text-sm",
                        "Save"
                    }
                }
            }
        }
    }
}
