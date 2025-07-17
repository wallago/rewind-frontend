use dioxus::prelude::*;

#[derive(Debug, Clone)]
struct HoverCardState(pub Signal<bool>);

#[derive(PartialEq, Clone, Props)]
pub struct HoverCardProps {
    children: Element,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub id: Option<String>,
}

#[component]
pub fn HoverCard(props: HoverCardProps) -> Element {
    let mut is_hovered = HoverCardState(Signal::new(false));
    use_context_provider(|| is_hovered.clone());

    rsx! {
        div {
            id: props.id,
            class: "relative inline-block",
            onmouseenter: move |_| is_hovered.0.set(true),
            onmouseleave: move |_| is_hovered.0.set(false),
            {props.children}
        }
    }
}

#[component]
pub fn HoverCardContent(props: HoverCardProps) -> Element {
    let is_hovered = use_context::<HoverCardState>();

    if !is_hovered.0() {
        return rsx!(
            div { id: props.id }
        );
    }
    rsx!(
        div {
            id: props.id,
            class: format!(
                "shadow-lg fixed z-50 mt-3 bg-primary border-2 border-secondary p-1 text-secondary {}",
                props.class.unwrap_or_default(),
            ),
            {props.children}
        }
    )
}
