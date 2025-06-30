use dioxus::prelude::*;
use wasm_bindgen::{JsCast, prelude::Closure};
use web_sys::{Document, HtmlElement};

struct ClickOutsideListener {
    document: Document,
    closure: Closure<dyn FnMut(web_sys::MouseEvent)>,
}

impl Drop for ClickOutsideListener {
    fn drop(&mut self) {
        self.document
            .remove_event_listener_with_callback("mousedown", self.closure.as_ref().unchecked_ref())
            .unwrap();

        tracing::info!("ClickOutsideListener dropped: event listener removed");
    }
}

pub fn use_click_outside<F>(id: String, is_active: F, on_outside: EventHandler)
where
    F: Fn() -> bool + 'static + Clone,
{
    let mut listener = use_signal(|| None::<ClickOutsideListener>);

    use_effect(move || {
        if !is_active() {
            tracing::info!("Not registering click-outside handler (inactive)");
            listener.set(None);
            return;
        }

        tracing::info!("Registering click-outside handler for {id}");

        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let document_for_closure = document.clone();

        let id_closure = id.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            tracing::info!("Click event caught");
            let target = event.target();

            if let Some(target) = target {
                let target: Option<HtmlElement> = target.dyn_into::<HtmlElement>().ok();
                let input_area = document_for_closure.get_element_by_id(&id_closure);

                if let (Some(target), Some(input_area)) = (target, input_area) {
                    tracing::info!("Input area found, target found");
                    if !input_area.contains(Some(&target)) {
                        tracing::info!("Clicked outside input area");
                        on_outside.call(())
                    }
                } else {
                    tracing::info!("Either target or input_area is None");
                }
            }
        }) as Box<dyn FnMut(_)>);

        document
            .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())
            .unwrap();

        listener.set(Some(ClickOutsideListener { document, closure }));
    });
}
