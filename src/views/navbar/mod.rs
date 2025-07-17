use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_regular_icons::FaNoteSticky;
use dioxus_free_icons::icons::fa_solid_icons::{FaChevronDown, FaMoon, FaPlus};

use crate::Route;
use crate::components::{
    Button, Dropdown, DropdownContent, DropdownTrigger, SearchDropdown, SearchDropdownContent,
    SearchDropdownInput,
};
use crate::context::{BoardsContext, ThemeContext};
use crate::hooks::use_click_outside;

mod modals;

#[component]
pub fn Navbar() -> Element {
    let mut dark_mode = use_context::<ThemeContext>();
    let ctx_boards = use_context::<BoardsContext>();

    let mut search = use_signal(|| "".to_string());

    let is_search_active = use_memo(move || !search().is_empty());
    let mut is_add_board_open = use_signal(|| false);
    let mut is_recent_boards_open = use_signal(|| false);

    let mut board_recent_options = use_signal(|| vec![]);
    let mut board_search_options = use_signal(|| vec![]);
    use_effect(move || {
        board_recent_options.set(
            (ctx_boards.boards)()
                .iter()
                .map(|board| {
                    let uuid = board.uuid.clone();
                    (
                        board.name.clone(),
                        Some(EventHandler::new(move |_| {
                            is_recent_boards_open.set(false);
                            navigator().push(Route::Board { uuid: uuid.clone() });
                        })),
                    )
                })
                .collect(),
        );
        board_search_options.set(
            (ctx_boards.boards)()
                .iter()
                .map(|board| board.name.clone())
                .collect(),
        );
    });

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

    let toggle_dark_mode = move |_| {
        dark_mode.0.toggle();
    };

    rsx! {
        div { class: "h-12 py-2 px-4 bg-primary flex items-center border-b-2 border-secondary",
            button {
                class: "px-2 py-1 text-secondary hover:text-secondary-2",
                onclick: |_| {
                    navigator().push(Route::Home {});
                },
                Icon { height: 24, icon: FaNoteSticky }
            }
            div { class: "pl-20 flex gap-12",
                Dropdown {
                    id: "recent-boards-area",
                    is_open: is_recent_boards_open,
                    class: "font-semibold text-base",
                    options: board_recent_options,
                    DropdownTrigger {
                        Icon { height: 14, width: 14, icon: FaChevronDown }
                    }
                    DropdownContent {}
                }
                Button {
                    class: "px-2 justify-between gap-2 font-semibold text-base",
                    onclick: move |_| is_add_board_open.set(true),
                    "Board"
                    Icon { height: 14, width: 14, icon: FaPlus }
                }
            }
            div { class: "ml-auto flex gap-12 items-center",
                SearchDropdown {
                    id: "search-boards-area",
                    options: board_search_options,
                    value: search,
                    class: "text-base w-72",
                    SearchDropdownInput { placeholder: " Search boards" }
                    SearchDropdownContent {}
                }
                button {
                    class: "px-2 py-1 text-secondary hover:text-secondary-2",
                    onclick: toggle_dark_mode,
                    Icon { height: 24, icon: FaMoon }
                }
            }
        }
        modals::AddBoard { is_open: is_add_board_open }
    }
}
