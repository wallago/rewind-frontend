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
    #[props(optional)]
    pub width: Option<String>,
    #[props(optional)]
    pub variant: Option<String>,
}

#[component]
pub fn Textarea(mut props: TextareaProps) -> Element {
    let variant_class = match props.variant.as_deref() {
        _ => {
            "
            border-2 border-secondary-2
            bg-primary-1 text-secondary
            placeholder:text-secondary-2
            focus:border-secondary
        "
        }
    };
    let base_class = "px-2 flex items-center focus:outline-none focus:ring-0";
    rsx!(textarea {
        class: format!(
            "{} {} {} {}",
            variant_class,
            base_class,
            props.class.unwrap_or_default(),
            props.width.clone().unwrap_or("w-fit".to_string()),
        ),
        id: props.id,
        placeholder: props.placeholder.unwrap_or("Enter".to_string()),
        name: props.name.unwrap_or_default(),
        rows: props.rows.unwrap_or("4".to_string()),
        cols: props.cols.unwrap_or("33".to_string()),
        value: (props.value)(),
        disabled: props.disabled.unwrap_or(false),
        oninput: move |e| { props.value.set(e.value()) },
    })
}
