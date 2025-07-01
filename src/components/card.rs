use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct CardProps {
    children: Element,
    #[props(optional)]
    class: Option<String>,
    #[props(optional)]
    pub width: Option<String>,
    #[props(optional)]
    pub id: Option<String>,
}

#[component]
pub fn Card(props: CardProps) -> Element {
    let base_class = "bg-primary-1 border-2 border-secondary p-3";
    rsx!(
        div {
            id: props.id,
            class: format!(
                "{} {} {}",
                base_class,
                props.class.clone().unwrap_or_default(),
                props.width.clone().unwrap_or("w-fit".to_string())
            ),
            {props.children}
        }
    )
}
