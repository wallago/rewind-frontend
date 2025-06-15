use dioxus::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::DomTokenList;

use crate::DarkMode;

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

const CHECKBOX_CSS: Asset = asset!("/assets/styling/checkbox.css");

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
        document::Link { rel: "stylesheet", href: CHECKBOX_CSS }
        label {
            class: "flex items-center gap-2 cursor-pointer",
            input {
                r#type: "checkbox",
                class: "hidden peer",
                checked: dark_mode().0,
                // onchange: move |e| {
                //     dark_mode.write().0 = (e.value() == "true");
                // }
                oninput: move |event| {
                    let is_enabled = event.value() == "true";
                    dark_mode.write().0 = is_enabled;
                }
            }
            // div {
            //     class: "w-4 h-4 rounded border border-gray-400 bg-white dark:bg-gray-800 flex items-center justify-center",
            //     div {
            //         class: "w-2 h-2 rounded bg-gray-800 dark:bg-white hidden",
            //     }
            // }
            span {
                class: "custom-checkbox w-4 h-4 border-2 border-zinc-700  transition-colors",
            }
        }
    }
}
