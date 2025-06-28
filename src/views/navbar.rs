use crate::{DarkMode, Route, helpers};
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_solid_icons::FaChevronDown;

#[component]
pub fn Navbar() -> Element {
    let mut dark_mode = use_context::<DarkMode>();

    use_effect(move || {
        let is_dark = dark_mode.0();
        tracing::info!("Setting dark mode: {:?}", is_dark);
        let dom_token_list = helpers::get_dom_token_list();
        if let Some(dom_token_list) = dom_token_list {
            if dark_mode.0() {
                let _ = dom_token_list.add_1("dark");
            } else {
                let _ = dom_token_list.remove_1("dark");
            }
        }
    });

    rsx! {
        div {
            class: "h-10 pt-2",
            div {
                div {
                    class:"flex justify-between my-2 px-4 font-semibold text-xs items-center",
                    div {
                        class:"flex gap-32",
                        div {
                            class: "flex items-center",
                            "Home router 1"
                                Icon {
                                    height: 10,
                                    icon: FaChevronDown,
                                }
                        }
                        div {
                            class: "flex gap-10",
                            div {
                                class: "bg-alert-primary p-2 rounded-sm text-primary",
                                "Alert mode"
                            }
                            div {
                                class: "bg-secondary-5 p-2 rounded-sm text-primary",
                                "AI analytics"
                            }
                        }
                    }
                    div {
                        class: "bg-secondary h-6 w-6 rounded-2xl",
                    }
                }
                // ThemeToggle {}
            }
            div {
                class: "px-6 bg-secondary text-primary text-[12px] h-10 flex items-center",
                "@ Copyrihgt-2023 - 2046"
            }
            // button {
            //     onclick: move |_| {
            //         dark_mode.0.toggle();
            //     },
            //     "what"
            // }
        }
    }
}
