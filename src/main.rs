use components::{Navbar, get_dom_token_list};
use dioxus::prelude::*;
use views::Home;

mod components;
mod views;

#[derive(Debug, Clone)]
struct DarkMode(bool);

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
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
            class: "dark:bg-zinc-900 bg-amber-50 dark:text-slate-200 text-slate-600 min-h-screen",
            Router::<Route> {}
        }
    }
}
