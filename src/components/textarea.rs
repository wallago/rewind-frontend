use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct TextareaProps {
    pub value: Signal<String>,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub placeholder: Option<String>,
    #[props(optional)]
    pub name: Option<String>,
    #[props(optional)]
    pub id: Option<String>,
    #[props(optional)]
    pub disabled: Option<bool>,
    #[props(optional)]
    pub rows: Option<String>,
    #[props(optional)]
    pub cols: Option<String>,
}

#[component]
pub fn Textarea(mut props: TextareaProps) -> Element {
    rsx!(textarea {
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
        id: props.id,
        placeholder: props.placeholder.unwrap_or("Enter".to_string()),
        name: props.name.unwrap_or_default(),
        rows: props.rows.unwrap_or("4".to_string()),
        cols: props.cols.unwrap_or("33".to_string()),
        value: (props.value)(),
        disabled: props.disabled.unwrap_or(false),
        oninput: move |e| { props.value.set(e.value()) }
    })
}
