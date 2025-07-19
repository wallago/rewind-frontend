use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct ButtonProps {
    children: Element,
    #[props(optional)]
    class: Option<String>,
    #[props(optional)]
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(optional)]
    r#type: Option<String>,
    #[props(optional)]
    pub disabled: Option<bool>,
    #[props(optional)]
    pub variant: Option<String>,
    #[props(optional)]
    pub width: Option<String>,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let variant_class = match props.variant.as_deref() {
        Some("outline") => {
            "
            bg-primary text-secondary
            hover:bg-primary-1 hover:text-secondary-1
            active:bg-primary-2 active:text-secondary-2
            border-2 border-secondary
            "
        }
        Some("ghost") => {
            "
            bg-transparent text-secondary
            hover:text-secondary-1
            active:text-secondary-2
            "
        }
        _ => {
            "
            bg-secondary text-primary
            hover:bg-secondary-1 hover:text-primary-1
            active:bg-secondary-2 active:text-primary-2
            "
        }
    };
    let disabled_class = if props.disabled.unwrap_or(false) {
        "pointer-events-none opacity-50"
    } else {
        ""
    };

    let base_class = " flex items-center";
    rsx!(
        button {
            class: format!(
                "{} {} {} {} {}",
                base_class,
                variant_class,
                props.class.clone().unwrap_or_default(),
                props.width.clone().unwrap_or("w-fit".to_string()),
                disabled_class,
            ),
            r#type: props.r#type.clone().unwrap_or("button".into()),
            disabled: props.disabled.unwrap_or(false),
            onclick: move |e: MouseEvent| {
                if let Some(handler) = &props.onclick {
                    handler.call(e);
                }
            },
            {props.children}
        }
    )
}
