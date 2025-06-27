use dioxus::prelude::*;
use views::{Boards, Lists, Tasks};

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
        Boards {},
        #[route("/board/:uuid")]
        Lists { uuid: String },
        #[route("/list/:uuid")]
        Tasks { uuid: String },
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
        let dom_token_list = get_dom_token_list();
        if let Some(dom_token_list) = dom_token_list {
            let is_dark = dom_token_list.contains("dark");
            if is_dark {
                DarkMode(Singal::new(true))
            } else {
                DarkMode(Signal::new(false))
            }
        } else {
            DarkMode(Signal::new(false))
        }
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "stylesheet", href: GLOBAL_CSS }
        div {
            class: "relative min-h-screen font-frida font-medium text-sm bg-primary text-secondary",
            Router::<Route> {}
        }
    }
}

#[component]
pub fn MainLayout() -> Element {
    rsx! {
        // Navbar {}
        div {
            class: "flex-grow pt-10",
            Outlet::<Route> {}
        }
        // Footer {}
    }
}
