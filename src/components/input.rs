use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct InputProps {
    pub value: Signal<String>,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub name: Option<String>,
    #[props(optional)]
    pub id: Option<String>,
    #[props(optional)]
    pub placeholder: Option<String>,
    #[props(optional)]
    pub disabled: Option<bool>,
}

#[component]
pub fn Input(mut props: InputProps) -> Element {
    rsx!(input {
        class: format!(
            "
            text-sm font-medium
            bg-surface-light dark:bg-surface-dark
            border-2 border-border-light dark:border-border-dark
            p-1
            focus:outline-none focus:ring-0
            {}
        ",
            props.class.unwrap_or_default()
        ),
        name: props.name,
        id: props.id,
        placeholder: props.placeholder.unwrap_or("Enter".to_string()),
        value: (props.value)(),
        disabled: props.disabled.unwrap_or(false),
        oninput: move |e| { props.value.set(e.value()) }
    })
}
