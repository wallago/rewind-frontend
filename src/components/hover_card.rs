use dioxus::prelude::*;

#[derive(Debug, Clone)]
struct HoverCardState(pub Signal<bool>);

#[component]
pub fn HoverCard(children: Element) -> Element {
    let mut is_hovered = HoverCardState(Signal::new(false));
    use_context_provider(|| is_hovered.clone());

    rsx! {
        div {
            class: "relative inline-block",
            onmouseenter: move |_| is_hovered.0.set(true),
            onmouseleave: move |_| is_hovered.0.set(false),
            {children}
        }
    }
}

#[component]
pub fn HoverCardContent(children: Element, class: Option<String>) -> Element {
    let is_hovered = use_context::<HoverCardState>();

    if !is_hovered.0() {
        return rsx!(); // or return None;
    }
    rsx!(
        div {
            class: format!("fixed z-50 mt-3 bg-primary border-2 border-secondary p-1 text-secondary {}", class.unwrap_or_default()),
            {children}
        }
    )
}
