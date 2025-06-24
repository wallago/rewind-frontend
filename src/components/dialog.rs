use super::Button;
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
struct DialogProps {
    children: Element,
    #[props(optional)]
    class: Option<String>,
    #[props(optional)]
    as_child: Option<bool>,
}

#[derive(Clone, Copy)]
struct DialogState(Signal<bool>);

#[component]
fn Dialog(props: DialogProps) -> Element {
    rsx!(
        div {
            class: "cursor-default",
            onclick: move |e: MouseEvent| {
                e.stop_propagation();
            },
            {props.children}
        }
    )
}

#[component]
fn DialogTrigger(props: DialogProps) -> Element {
    let mut is_open = use_context::<DialogState>().0;
    if props.as_child.unwrap_or(false) {
        rsx! {
            div {
                 onclick: move |_| {
                     is_open.set(true)
                 },
                {props.children}
            }
        }
    } else {
        rsx!(
            Button {
                 onclick: Some(EventHandler::new(move |e: MouseEvent| {
                     is_open.set(true)
                 })),
                {props.children}
            }
        )
    }
}

#[component]
fn DialogContent(props: DialogProps) -> Element {
    let is_open = use_context::<DialogState>().0;

    if !is_open() {
        return rsx!();
    }
    rsx!(
        div {
            class: "fixed inset-0 z-50 flex items-center justify-center bg-black/50",
            div {
                class: format!("
                   bg-surface-light dark:bg-surface-dark
                   shadow-xl py-4 px-6 w-full max-w-md flex flex-col 
                   text-left
               {}", props.class.clone().unwrap_or_default()),
                {props.children}
            }
        }
    )
}

#[component]
fn DialogClose(props: DialogProps) -> Element {
    let mut is_open = use_context::<DialogState>().0;

    if props.as_child.unwrap_or(false) {
        rsx!(
            div {
                 onclick: move |e: MouseEvent| {
                     is_open.set(false)
                 },
                {props.children}
            }
        )
    } else {
        rsx!(
            Button {
                 onclick: Some(EventHandler::new(move |e: MouseEvent| {
                     is_open.set(false)
                 })),
                "Cancel"
            }
        )
    }
}

#[component]
fn DialogDescription(props: DialogProps) -> Element {
    rsx!(
        p {
            class: "text-sm font-medium text-text-light/70 dark:text-text-dark/70",
            {props.children}
        }
    )
}

#[component]
fn DialogFooter(props: DialogProps) -> Element {
    rsx!(
        div {
            class: "mt-2 flex justify-end gap-4",
            {props.children}
        }
    )
}

#[component]
fn DialogHeader(props: DialogProps) -> Element {
    rsx!(
        div {
            class: "space-y-1",
            {props.children}
        }
    )
}

#[component]
fn DialogTitle(props: DialogProps) -> Element {
    rsx!(
        h2 {
            class: "text-lg font-semibold",
            {props.children}
        }
    )
}

#[derive(PartialEq, Props, Clone)]
pub struct DialogFormProps {
    title: String,
    description: Option<String>,
    trigger: Element,
    form: Element,
    #[props(optional)]
    submit: Option<EventHandler<FormEvent>>,
}

#[component]
pub fn DialogForm(props: DialogFormProps) -> Element {
    let mut is_open = use_signal(|| false);
    use_context_provider(|| DialogState(is_open));

    rsx!(
        Dialog {
            DialogTrigger {
                as_child: Some(true),
                {props.trigger}
            }
            form {
                onsubmit: move |e| {
                    // evt.prevent_default(); // reload
                    if let Some(handler) = &props.submit {
                        handler.call(e);
                    }
                    is_open.set(false);
                },
                DialogContent {
                    class: "sm:max-w-[425px]",
                    DialogHeader {
                        DialogTitle { {props.title} }
                        DialogDescription { {props.description} }
                    }
                    div {
                        class: "grid gap-4 my-4",
                        {props.form}
                    }
                    DialogFooter {
                        DialogClose {}
                        Button {
                            r#type:"submit",
                            variant: "outline",
                            class: "font-semibold",
                            "Save"
                        }
                    }
                }
            }
        }
    )
}

#[derive(PartialEq, Props, Clone)]
pub struct DialogSimpleProps {
    title: String,
    description: Option<String>,
    trigger: Element,
    submit: Option<EventHandler<MouseEvent>>,
}

#[component]
pub fn DialogSimple(props: DialogSimpleProps) -> Element {
    let mut is_open = use_signal(|| false);
    use_context_provider(|| DialogState(is_open));

    rsx!(
        Dialog {
            DialogTrigger {
                as_child: Some(true),
                {props.trigger}
            }
            DialogContent {
                class: "sm:max-w-[425px]",
                DialogHeader {
                    DialogTitle { {props.title} }
                    DialogDescription { {props.description} }
                }
                DialogFooter {
                    DialogClose {}
                    Button {
                        r#type:"submit",
                        variant: "outline",
                        onclick: Some(EventHandler::new(move |e: MouseEvent| {
                            if let Some(handler) = &props.submit {
                                handler.call(e);
                            }
                            is_open.set(false);
                        })),
                        class: "font-semibold",
                        "Save"
                    }
                }
            }
        }
    )
}
