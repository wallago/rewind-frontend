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
    #[props(optional)]
    pub width: Option<String>,
    #[props(optional)]
    pub onenter: Option<EventHandler<KeyboardEvent>>,
}

#[component]
pub fn Input(mut props: InputProps) -> Element {
    let variant_class = match props.variant.as_deref() {
        _ => {
            "
            border-2 border-secondary-4 
            bg-primary-2 text-secondary
            placeholder:text-secondary-4
            focus:border-secondary
        "
        }
    };
    let base_class = "px-2 flex items-center focus:outline-none focus:ring-0";
    rsx!(input {
        class: format!(
            "{} {} {} {}",
            variant_class,
            base_class,
            props.class.unwrap_or_default(),
            props.width.clone().unwrap_or("w-fit".to_string())
        ),
        type: props.r#type,
        name: props.name,
        id: props.id,
        placeholder: props.placeholder.unwrap_or("Enter".to_string()),
        disabled: props.disabled.unwrap_or(false),
        value: (props.value)(),
        oninput: move |e| { props.value.set(e.value()) },
        onkeydown: move |e: KeyboardEvent| {
            if e.key() == Key::Enter &&  !(props.value)().is_empty() {
                if let Some(handler) = &props.onenter {
                    handler.call(e);
                }
            }
        }
    })
}
