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
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let variant_class = match props.variant.as_deref() {
        Some("outline") => {
            "
            border-2 border-border-light dark:border-border-dark
            bg-surface-light dark:bg-surface-dark 
            text-text-light dark:text-text-dark 
            fill-text-light dark:fill-text-dark
            hover:bg-surface-light/80 dark:hover:bg-surface-dark/80
            active:bg-surface-light/50 dark:active:bg-surface-dark/50
            "
        }
        // Some("ghost") => "hover:bg-accent hover:text-accent-foreground",
        // Some("destructive") => "bg-destructive text-destructive-foreground hover:bg-destructive/90",
        _ => {
            "
            bg-text-light dark:bg-text-dark 
            text-surface-light dark:text-surface-dark 
            fill-surface-light dark:fill-surface-dark
            hover:bg-text-light/80 dark:hover:bg-text-dark/80
            active:bg-text-light/50 dark:active:bg-text-dark/50
            "
        }
    };

    let base_class = "p-1 font-medium text-sm transition-colors duration-200";
    rsx!(
        button {
            class: format!(
                "{} {} {}",
                base_class,
                variant_class,
                props.class.clone().unwrap_or_default()
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
