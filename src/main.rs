use components::{Footer, Navbar, get_dom_token_list};
use dioxus::prelude::*;
use views::Boards;

mod components;
mod views;

#[derive(Debug, Clone)]
struct DarkMode(bool);

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(MainLayout)]
        #[route("/")]
        Boards {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
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

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        div {
            class: "
                relative min-h-screen 
                font-frida 
                dark:bg-surface-dark bg-surface-light 
                dark:text-text-dark text-text-light 
            ",
                // min-h-screen min-w-screen
            Router::<Route> {}
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
