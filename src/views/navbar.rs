use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_regular_icons::FaNoteSticky;
use dioxus_free_icons::icons::fa_solid_icons::{FaChevronDown, FaMoon, FaPlus};

use crate::{
    DarkMode,
    components::{Button, Input},
    helpers,
};

#[component]
pub fn Navbar() -> Element {
    let mut dark_mode = use_context::<DarkMode>();
    let search = Signal::new("".to_string());

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
                Button {
                    class: "pr-1 pl-2 w-24 justify-between font-semibold text-sm",
                    width: "w-24",
                    "Recent"
                    Icon {
                        height: 14,
                        icon: FaChevronDown,
                    }
                }
                Button {
                    class: "pr-1 pl-2 justify-between font-semibold text-sm",
                    width: "w-24",
                    "Board"
                    Icon {
                        height: 14,
                        icon: FaPlus,
                    }
                }
            }
            div {
                class: "ml-auto flex gap-12 items-center",
                Input {
                    class: "h-6 justify-between text-xs",
                    placeholder: " Search",
                    value: search,
                    width: "w-32"
                }
                button {
                    class: "px-2 py-1 text-secondary hover:text-secondary-2",
                    onclick: toggle_dark_mode,
                    Icon {
                        height: 24,
                        icon: FaMoon,
                    }
                }
            }
        }
    }
}
