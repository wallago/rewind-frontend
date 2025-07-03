use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_regular_icons::FaNoteSticky;
use dioxus_free_icons::icons::fa_solid_icons::{FaChevronDown, FaMoon, FaPlus};

use crate::Route;
use crate::api::add_board;
use crate::hooks::use_click_outside;
use crate::models::NewBoard;
use crate::{
    DarkMode,
    components::{
        Button, Dialog, DialogClose, DialogContent, DialogFooter, DialogHeader, DialogTitle,
        Dropdown, DropdownContent, DropdownTrigger, Input, SearchDropdown, SearchDropdownContent,
        SearchDropdownInput,
    },
    helpers,
};

#[component]
pub fn Navbar() -> Element {
    let mut dark_mode = use_context::<DarkMode>();
    let mut search = use_signal(|| "".to_string());

    let is_search_active = use_memo(move || !search().is_empty());
    let mut is_add_board_open = use_signal(|| false);
    let mut is_recent_boards_open = use_signal(|| false);

    use_click_outside(
        "add-board-area".to_string(),
        move || is_add_board_open(),
        EventHandler::new(move |_| is_add_board_open.set(false)),
    );

    use_click_outside(
        "recent-boards-area".to_string(),
        move || is_recent_boards_open(),
        EventHandler::new(move |_| is_recent_boards_open.set(false)),
    );

    use_click_outside(
        "search-boards-area".to_string(),
        move || is_search_active(),
        EventHandler::new(move |_| search.set("".to_string())),
    );

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
            button {
                class: "px-2 py-1 text-secondary hover:text-secondary-2",
                onclick: |_| {
                    navigator().push(Route::Home {});
                },
                Icon { height: 24, icon: FaNoteSticky }
            }
            div {
                class: "pl-20 flex gap-12",
                Dropdown {
                    id: "recent-boards-area",
                    is_open: is_recent_boards_open,
                    class: "font-semibold text-base",
                    options: [
                        (String::from("Project X"), Some(EventHandler::new(move |_| {
                            is_recent_boards_open.set(false);
                            navigator().push(Route::Board { uuid: String::from("571a9fa0-1bb4-4545-bdd3-b7315dcb6615") });
                        }))),
                        (String::from("Project Y"), None),
                        (String::from("Project Z"), None)
                    ].to_vec(),
                    DropdownTrigger {
                        Icon { height: 14, width: 14,icon: FaChevronDown }
                    }
                    DropdownContent {}
                }
                Button {
                    class: "px-2 justify-between gap-2 font-semibold text-base",
                    width: "w-24",
                    onclick: move |_| is_add_board_open.set(true),
                    "Board"
                    Icon { height: 14, width: 14,icon: FaPlus }
                }
            }
            div {
                class: "ml-auto flex gap-12 items-center",
                SearchDropdown {
                    id: "search-boards-area",
                    options: Signal::new(Vec::new()),
                    value: search,
                    class: "text-base w-72",
                    SearchDropdownInput {
                        placeholder: " Search boards",
                    }
                    SearchDropdownContent {}
                }
                button {
                    class: "px-2 py-1 text-secondary hover:text-secondary-2",
                    onclick: toggle_dark_mode,
                    Icon { height: 24, icon: FaMoon }
                }
            }
        }
        AddBoard { is_open: is_add_board_open }
    }
}

#[component]
fn AddBoard(is_open: Signal<bool>) -> Element {
    let name = use_signal(|| "".to_string());
    rsx! {
        Dialog {
            is_open: is_open,
            DialogContent {
                id: "add-board-area",
                class: "sm:max-w-[425px]",
                DialogHeader {
                    DialogTitle { "Add Board" }
                }
                Input {
                    width: "w-full",
                    placeholder: "Enter board name",
                    value: name,
                }
                DialogFooter {
                    DialogClose {}
                    Button {
                        onclick: move |_| {
                            use_future(move || async move {
                                match add_board(NewBoard {
                                    name: name(),
                                    position: None
                                }).await {
                                    Ok(_) => (),
                                    Err(err) => tracing::error!("{err}"),

                                }
                            });
                            navigator().push(Route::Home {});
                            is_open.set(false);
                        },
                        r#type:"submit",
                        variant: "outline",
                        class: "font-semibold px-2 text-sm",
                        "Save"
                    }
                }
            }
        }
    }
}
