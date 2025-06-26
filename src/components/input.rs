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
    #[props(optional)]
    pub r#type: Option<String>,
    #[props(optional)]
    pub variant: Option<String>,
}

#[component]
pub fn Input(mut props: InputProps) -> Element {
    let class = match props.variant.as_deref() {
        Some("variant") => {
            "
            bg-surface-variant-light dark:bg-surface-variant-dark
        "
        }
        _ => {
            "
            bg-surface-light dark:bg-surface-dark
        "
        }
    };
    rsx!(input {
        class: format!(
            "
            text-sm font-medium
            placeholder:text-border-light dark:placeholder:text-border-dark
            border-2 border-border-light dark:border-border-dark
            p-1
            focus:outline-none focus:ring-0
            {} {}
        ",
            class,
            props.class.unwrap_or_default()
        ),
        type: props.r#type,
        name: props.name,
        id: props.id,
        placeholder: props.placeholder.unwrap_or("Enter".to_string()),
        disabled: props.disabled.unwrap_or(false),
        value: (props.value)(),
        onchange: move |e| { props.value.set(e.value()) }
    })
}
