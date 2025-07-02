use dioxus::prelude::*;

use crate::components::Button;

#[derive(Clone)]
struct DropdownContext {
    is_open: Signal<bool>,
    options: Vec<(String, Option<EventHandler<MouseEvent>>)>,
}

#[derive(PartialEq, Clone, Props)]
pub struct DropdownProps {
    pub options: Vec<(String, Option<EventHandler<MouseEvent>>)>,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub id: Option<String>,
    #[props(optional)]
    is_open: Option<Signal<bool>>,
    pub children: Element,
}

#[component]
pub fn Dropdown(props: DropdownProps) -> Element {
    use_context_provider(|| DropdownContext {
        is_open: props.is_open.unwrap_or(Signal::new(false)),
        options: props.options,
    });

    let base_class = "relative w-full";
    rsx!(div {
        id: props.id,
        class: format!("{} {}", base_class, props.class.clone().unwrap_or_default()),
        {props.children}
    })
}

#[derive(PartialEq, Clone, Props)]
pub struct DropdownTriggerProps {
    #[props(optional)]
    pub name: Option<String>,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub id: Option<String>,
    #[props(optional)]
    pub width: Option<String>,
    pub children: Element,
}

#[component]
pub fn DropdownTrigger(props: DropdownTriggerProps) -> Element {
    let mut ctx = use_context::<DropdownContext>();
    rsx!(Button {
            onclick: move |_| {
                ctx.is_open.toggle();
            },
            class: format!("px-2 flex gap-2 justify-between items-center {}", props.width.unwrap_or("w-fit".to_string())),
            span {
                class: "truncate",
                {props.name.clone().unwrap_or(String::from("Select an option"))}
            }
            {props.children}
    })
}

#[derive(PartialEq, Clone, Props)]
pub struct DropdownContentProps {
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub id: Option<String>,
    #[props(optional)]
    pub width: Option<String>,
    #[props(optional)]
    pub onclick: Option<EventHandler<MouseEvent>>,
}

#[component]
pub fn DropdownContent(props: DropdownContentProps) -> Element {
    let mut ctx = use_context::<DropdownContext>();
    if (ctx.is_open)() {
        rsx!(
            div {
                id: props.id,
                class: format!("
                    absolute z-50 mt-1 max-h-60 overflow-y-auto
                    border-2 border-secondary shadow-lg
                    bg-primary text-secondary
                    {} {}", props.class.unwrap_or_default(), props.width.unwrap_or("w-full".to_string())),
                    {ctx.options.into_iter().map(|(label,handler)| {
                        rsx!(button {
                            class: "w-full text-left px-1 hover:bg-primary-1 hover:text-secondary-1",
                            onclick: move |e: MouseEvent| {
                                if let Some(handler) = handler {
                                    handler.call(e)
                                }
                                ctx.is_open.set(false);
                            },
                            "{label}"
                        })
                    })}
            }
        )
    } else {
        rsx!()
    }
}
