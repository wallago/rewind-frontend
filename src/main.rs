use components::{Footer, Navbar, get_dom_token_list, modal};
use dioxus::prelude::*;
use views::Home;

mod components;
mod views;

#[derive(Debug, Clone)]
struct DarkMode(bool);

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(MainLayout)]
        #[route("/")]
        Home {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_context_provider(|| modal::ModalState {
        is_open: Signal::new(false),
    });

    use_context_provider(|| {
        let dom_token_list = get_dom_token_list();
        if let Some(dom_token_list) = dom_token_list {
            let is_dark = dom_token_list.contains("dark");
            if is_dark {
                Signal::new(DarkMode(true))
            } else {
                Signal::new(DarkMode(false))
            }
        } else {
            Signal::new(DarkMode(false))
        }
    });

    let modal_state = use_context::<modal::ModalState>();

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        div {
            class: "font-frida dark:bg-surface-dark bg-surface-light dark:text-text-dark text-text-light min-h-screen",
            Router::<Route> {}
            if (modal_state.is_open)() {
                modal::Modal {}
            }
        }
    }
}

#[component]
pub fn MainLayout() -> Element {
    rsx! {
        Navbar {}
        div {
            class: "flex-grow",
            Outlet::<Route> {}
        }
        Footer {}
    }
}
