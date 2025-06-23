use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct ClickableProps {
    children: Element,
    #[props(optional)]
    class: Option<String>,
    #[props(optional)]
    onclick: Option<EventHandler<MouseEvent>>,
}

#[component]
pub fn Button(props: ClickableProps) -> Element {
    let mut active = use_signal(|| false);
    let default_classes = "
        button-darken-effect
        p-1
        font-medium text-xs
        bg-surface-light dark:bg-surface-dark
        border-2 border-border-light dark:border-border-dark
        hover:bg-surface-light/50 dark:hover:bg-surface-dark/50
        active:bg-border-light dark:active:bg-border-dark
        transition-colors duration-200
    ";

    let onclick = {
        move |evt| {
            active.set(true);
            if let Some(on_user_click) = &props.onclick {
                on_user_click.call(evt);
            }
        }
    };
    rsx!(
        button {
            onclick: onclick,
            class: format!(
                "{} {} {}",
                props.class.clone().unwrap_or_default(),
                default_classes,
                if *active.read() { " active" } else { "" }
            ),
            {props.children}
        }
    )
}
