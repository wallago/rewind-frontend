use wasm_bindgen::JsCast;
use web_sys::{DomTokenList, HtmlElement, Storage, window};

pub fn get_dom_token_list() -> Option<DomTokenList> {
    let document = web_sys::window()?.document()?;
    let element = document.document_element()?;
    let html: HtmlElement = element.dyn_into().ok()?;
    let class_list = html.class_list();
    Some(class_list)
}

pub fn save_dark_mode_preference(is_dark: bool) {
    if let Some(storage) = window().and_then(|w| w.local_storage().ok()).flatten() {
        let _ = storage.set_item("theme", if is_dark { "dark" } else { "light" });
    }
}

pub fn load_dark_mode_preference() -> Option<bool> {
    let storage = window().and_then(|w| w.local_storage().ok()).flatten()?;
    match storage.get_item("theme").ok().flatten()?.as_str() {
        "dark" => Some(true),
        "light" => Some(false),
        _ => None,
    }
}
