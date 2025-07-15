use dioxus::prelude::*;

use crate::components::Input;

#[derive(Clone, Copy)]
struct SearchDropdownContext {
    input: Signal<String>,
    filtered_results: Memo<Vec<String>>,
    is_focus: Signal<bool>,
}

#[derive(PartialEq, Clone, Props)]
pub struct SearchDropdownProps {
    pub value: Signal<String>,
    pub options: Signal<Vec<String>>,
    #[props(optional)]
    pub id: Option<String>,
    #[props(optional)]
    pub class: Option<String>,
    pub children: Element,
}

#[component]
pub fn SearchDropdown(props: SearchDropdownProps) -> Element {
    let filtered_results = use_memo(move || {
        (props.options)()
            .iter()
            .cloned()
            .filter(|result| {
                if result
                    .to_lowercase()
                    .contains(&(props.value)().to_lowercase())
                {
                    true
                // } else if (props.value)() == "".to_string() {
                //     true
                } else {
                    false
                }
            })
            .collect::<Vec<String>>()
    });

    use_context_provider(|| SearchDropdownContext {
        filtered_results,
        input: props.value,
        is_focus: Signal::new(false),
    });

    let base_class = "relative w-full";
    rsx!(
        div {
            class: format!("{} {}", base_class, props.class.unwrap_or_default()),
            id: props.id,
            {props.children}
        }
    )
}

#[derive(PartialEq, Clone, Props)]
pub struct SearchDropdownInputProps {
    #[props(optional)]
    pub placeholder: Option<String>,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub id: Option<String>,
    #[props(optional)]
    pub width: Option<String>,
}

#[component]
pub fn SearchDropdownInput(props: SearchDropdownInputProps) -> Element {
    let ctx = use_context::<SearchDropdownContext>();
    rsx!(Input {
        id: props.id,
        class: props.class,
        placeholder: props.placeholder,
        value: ctx.input,
        width: props.width,
        is_focus: ctx.is_focus,
    })
}

#[derive(PartialEq, Clone, Props)]
pub struct SearchDropdownContentProps {
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
pub fn SearchDropdownContent(props: SearchDropdownContentProps) -> Element {
    let mut ctx = use_context::<SearchDropdownContext>();
    if (!(ctx.input)().is_empty() || (ctx.is_focus)()) && !ctx.filtered_results.is_empty() {
        rsx!(
            div {
                id: props.id,
                class: format!(
                    "
                                absolute z-50 mt-1 max-h-60 overflow-y-auto
                                border-2 border-secondary shadow-lg
                                bg-primary text-secondary
                                {} {}",
                    props.class.unwrap_or_default(),
                    props.width.unwrap_or("w-full".to_string()),
                ),
                {
                    ctx.filtered_results
                        .iter()
                        .map(|filter_result| {
                            rsx! {
                                button {
                                    class: "w-full text-left px-1 py-0.5 hover:bg-primary-1 hover:text-secondary-1",
                                    onclick: move |e: MouseEvent| {
                                        if let Some(handler) = props.onclick {
                                            handler.call(e)
                                        }
                                        ctx.input.set("".to_string())
                                    },
                                    "{filter_result}"
                                }
                            }
                        })
                }
            }
        )
    } else {
        rsx!()
    }
}
