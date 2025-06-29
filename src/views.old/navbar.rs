use crate::{DarkMode, helpers};
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_regular_icons::FaNoteSticky;
use dioxus_free_icons::icons::fa_solid_icons::{FaChevronDown, FaCircleHalfStroke, FaPlus};

#[component]
pub fn Navbar() -> Element {
    let mut dark_mode = use_context::<DarkMode>();

    use_effect(move || {
        let is_dark = dark_mode.0();
        tracing::info!("Setting dark mode: {:?}", is_dark);
        helpers::save_dark_mode_preference(is_dark);
        let dom_token_list = helpers::get_dom_token_list();
        if let Some(dom_token_list) = dom_token_list {
            if is_dark {
                let _ = dom_token_list.add_1("dark");
            } else {
                let _ = dom_token_list.remove_1("dark");
            }
        }
    });

    let toggle_dark_mode = move |_| {
        dark_mode.0.toggle();
    };

    rsx! {
        div {
            class: "h-12 py-2 px-4 bg-primary flex items-center border-b-2 border-secondary",
            div {
                class: "text-secondary",
                Icon {
                    height: 24,
                    icon: FaNoteSticky,
                }
            }
            div {
                class: "pl-20 flex gap-12",
                button {
                    class: "w-24 flex justify-between bg-secondary text-primary items-center pl-2 pr-1 font-semibold text-sm py-0.5 hover:bg-secondary-1 hover:text-primary-1",
                    "Recent"
                    Icon {
                        height: 14,
                        icon: FaChevronDown,
                    }
                }
                button {
                    class: "w-24 flex justify-between bg-secondary text-primary items-center pl-2 pr-1 font-semibold text-sm py-0.5 hover:bg-secondary-1 hover:text-primary-1",
                    "Board"
                    Icon {
                        height: 14,
                        icon: FaPlus,
                    }
                }
            }
            div {
                class: "ml-auto flex gap-12 items-center",
                input {
                    class: "h-6 w-32 flex justify-between border-2 border-secondary-4 bg-primary-2 text-secondary items-center px-2 text-xs placeholder:text-secondary-4 focus:outline-none focus:ring-0 focus:border-secondary",
                    placeholder: " Search"
                }
                button {
                    class: "px-2 py-1 text-secondary hover:text-secondary-2",
                    onclick: toggle_dark_mode,
                    Icon {
                        height: 24,
                        icon: FaCircleHalfStroke,
                    }
                }
            }
        }
    }
}
