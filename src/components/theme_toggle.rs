use dioxus::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::DomTokenList;

use crate::{DarkMode, components::icons::Contrast};

pub fn get_dom_token_list() -> Option<DomTokenList> {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Some(element) = document.document_element() {
                if let Ok(html) = element.dyn_into::<web_sys::HtmlElement>() {
                    let class_list = html.class_list();
                    return Some(class_list);
                }
            }
        }
    }
    None
}

#[component]
pub fn ThemeToggle() -> Element {
    let mut dark_mode = use_context::<Signal<DarkMode>>();

    use_effect(move || {
        let is_dark = dark_mode.read().0;
        tracing::info!("Setting dark mode: {:?}", is_dark);
        let dom_token_list = get_dom_token_list();
        if let Some(dom_token_list) = dom_token_list {
            if dark_mode().0 {
                let _ = dom_token_list.add_1("dark");
            } else {
                let _ = dom_token_list.remove_1("dark");
            }
        }
    });

    rsx! {
        button {
            class: "self-center w-5 h-5 hover:bg-muted-light dark:hover:bg-muted-dark",
            onclick: move |_| {
                dark_mode.write().0 = !dark_mode().0;
            },
            Contrast {}
        }
    }
}
