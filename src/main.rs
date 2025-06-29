use dioxus::prelude::*;
use views::{Board, Footer, Home, Navbar};

use crate::helpers::get_dom_token_list;

mod components;
mod helpers;
mod views;

#[derive(Debug, Clone)]
struct DarkMode(Signal<bool>);

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(MainLayout)]
        #[route("/")]
        Home {},
        #[route("/board/:uuid")]
        Board { uuid: String },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const GLOBAL_CSS: Asset = asset!("/assets/global.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_context_provider(|| {
        if let Some(is_dark) = helpers::load_dark_mode_preference() {
            DarkMode(Signal::new(is_dark))
        } else {
            let dom_token_list = get_dom_token_list();
            if let Some(dom_token_list) = dom_token_list {
                DarkMode(Signal::new(dom_token_list.contains("dark")))
            } else {
                DarkMode(Signal::new(false))
            }
        }
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "stylesheet", href: GLOBAL_CSS }
        div {
            Router::<Route> {}
        }
    }
}

#[component]
pub fn MainLayout() -> Element {
    rsx! {
        div {
            class: "h-screen overflow-hidden bg-primary-2",
            div {
                class: "fixed top-0 left-0 right-0 z-10",
                Navbar {}
            }
            div {
                class: "fixed bottom-0 left-0 right-0 z-10",
                Footer {}
            }
            div {
                class: "flex py-16 px-4 h-full",
                div {
                    class: "flex-grow overflow-auto p-4",
                    Outlet::<Route> {}
                }
            }
        }
    }
}
