use super::Button;
use dioxus::prelude::*;

#[derive(Clone, Copy)]
pub struct DialogContext {
    is_open: Signal<bool>,
}

#[derive(PartialEq, Props, Clone)]
pub struct DialogProps {
    children: Element,
    #[props(optional)]
    class: Option<String>,
    #[props(optional)]
    as_child: Option<bool>,
    #[props(optional)]
    id: Option<String>,
    #[props(optional)]
    is_open: Option<Signal<bool>>,
}

#[component]
pub fn Dialog(props: DialogProps) -> Element {
    use_context_provider(|| DialogContext {
        is_open: props.is_open.unwrap_or(Signal::new(false)),
    });
    rsx!(
        div {
            id: props.id.clone().unwrap_or_default(),
            class: "cursor-default",
            onclick: move |e: MouseEvent| {
                e.stop_propagation();
            },
            {props.children}
        }
    )
}

#[component]
pub fn DialogTrigger(props: DialogProps) -> Element {
    let mut ctx = use_context::<DialogContext>();
    if props.as_child.unwrap_or(false) {
        rsx! {
            div {
                id: props.id.clone().unwrap_or_default(),
                onclick: move |_| {
                    ctx.is_open.set(true)
                },
                {props.children}
            }
        }
    } else {
        rsx!(
            Button {
                onclick: Some(EventHandler::new(move |_| {
                    ctx.is_open.set(true)
                })),
                {props.children}
            }
        )
    }
}

#[component]
pub fn DialogContent(props: DialogProps) -> Element {
    let ctx = use_context::<DialogContext>();
    if !(ctx.is_open)() {
        return rsx!();
    }
    let base_class = "bg-primary shadow-xl py-4 px-6 w-full max-w-md flex flex-col gap-2 text-left border-2 border-secondary text-secondary";
    rsx!(
        div {
            class: "fixed inset-0 z-50 flex items-center justify-center bg-black/50",
            div {
                id: props.id.clone().unwrap_or_default(),
                class: format!("{} {}", base_class, props.class.clone().unwrap_or_default()),
                {props.children}
            }
    }
    )
}

#[component]
pub fn DialogClose(props: DialogProps) -> Element {
    let mut ctx = use_context::<DialogContext>();
    if props.as_child.unwrap_or(false) {
        rsx!(
            div {
                id: props.id.clone().unwrap_or_default(),
                onclick: move |_| {
                    ctx.is_open.set(false)
                },
                {props.children}
            }
        )
    } else {
        rsx!(
            Button {
                onclick: Some(EventHandler::new(move |_| {
                    ctx.is_open.set(false)
                })),
                class: "px-2 text-sm",
                "Cancel"
            }
        )
    }
}

#[component]
pub fn DialogDescription(props: DialogProps) -> Element {
    rsx!(
        p {
            class: "text-sm font-medium text-secondary-2",
            {props.children}
        }
    )
}

#[component]
pub fn DialogFooter(props: DialogProps) -> Element {
    rsx!(
        div {
            class: "mt-2 flex justify-end gap-4",
            {props.children}
        }
    )
}

#[component]
pub fn DialogHeader(props: DialogProps) -> Element {
    rsx!(
        div {
            class: "space-y-1",
            {props.children}
        }
    )
}

#[component]
pub fn DialogTitle(props: DialogProps) -> Element {
    rsx!(
        h2 {
            class: "text-lg font-semibold",
            {props.children}
        }
    )
}
