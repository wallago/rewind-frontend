use wasm_bindgen::JsCast;
use web_sys::{DomTokenList, HtmlElement};

pub fn get_dom_token_list() -> Option<DomTokenList> {
    let document = web_sys::window()?.document()?;
    let element = document.document_element()?;
    let html: HtmlElement = element.dyn_into().ok()?;
    let class_list = html.class_list();
    tracing::info!("Class list {:#?}", class_list.value());
    Some(class_list)
}
