use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_solid_icons::{FaChevronDown, FaMoon, FaPlus};

use crate::components::Button;

#[derive(PartialEq, Clone, Props)]
pub struct SelectProps {
    pub selected: Signal<Option<String>>,
    pub options: Vec<(String, String)>,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub name: Option<String>,
    #[props(optional)]
    pub id: Option<String>,
    #[props(optional)]
    pub width: Option<String>,
}

#[component]
pub fn Select(mut props: SelectProps) -> Element {
    let mut is_open = use_signal(|| false);
    let base_class = "w-full";
    rsx!(
        div { class: format!("{} {}", base_class, props.class.clone().unwrap_or_default()),
            Button {
                onclick: move |_| is_open.set(!is_open()),
                div {
                    class: format!("px-2 flex gap-2 justify-between items-center {}", props.width.unwrap_or("w-fit".to_string())),
                    {(props.selected)().unwrap_or("--Please choose an option--".to_string())}
                    Icon { height: 14, width: 14,icon: FaChevronDown }

                }
            }
            if is_open() {
                div {
                    class: "
                        mt-0.5 w-full
                        border-2 border-secondary
                        bg-surface-light dark:bg-surface-dark
                    ",
                    for (value, label) in props.options {
                        div {
                            onclick: move |_| {
                                props.selected.set(Some(value.clone()));
                                is_open.set(false);
                            },
                            class: "w-full p-1 hover:bg-primary cursor-pointer",
                            "{label}"
                        }
                    }
                }
            }
        }
    )
}
