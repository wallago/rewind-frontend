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
        _ => {
            "
            bg-secondary text-primary
            hover:bg-secondary-1 hover:text-primary-1
            active:bg-secondary-2 hover:text-primary-2
            "
        }
    };

    let base_class = " flex items-center px-2 py-0.5";
    rsx!(
        button {
            class: format!(
                "{} {} {} {}",
                base_class,
                variant_class,
                props.class.clone().unwrap_or_default(),
                props.width.clone().unwrap_or("w-fit".to_string())
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
