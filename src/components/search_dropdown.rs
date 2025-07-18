use std::sync::Arc;

use crate::{components::Input, hooks::use_click_outside};
use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::fa_solid_icons::FaCheck};

#[derive(Clone, Copy)]
struct SearchDropdownContext<T>
where
    T: Clone + PartialEq + 'static,
{
    input: Signal<String>,
    filtered_results: Memo<Vec<T>>,
    is_open: Signal<bool>,
    selected_options: Memo<Vec<T>>,
    selected_option: Signal<Option<T>>,
    get_label: fn(&T) -> String,
}

#[derive(PartialEq, Clone, Props)]
pub struct SearchDropdownProps<T>
where
    T: Clone + PartialEq + 'static,
{
    pub value: Signal<String>,
    pub is_open: Signal<bool>,
    pub options: Memo<Vec<T>>,
    pub get_label: fn(&T) -> String,
    #[props(optional)]
    pub id: Option<String>,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub selected_options: Option<Memo<Vec<T>>>,
    #[props(optional)]
    pub selected_option: Option<Signal<Option<T>>>,
    pub children: Element,
}

#[component]
pub fn SearchDropdown<T>(mut props: SearchDropdownProps<T>) -> Element
where
    T: Clone + PartialEq + 'static,
{
    let filtered_results = use_memo(move || {
        (props.options)()
            .into_iter()
            .filter(|item| {
                (props.get_label)(item)
                    .to_lowercase()
                    .contains(&(props.value)().to_lowercase())
            })
            // .filter(|result| {
            //     if result
            //         .to_lowercase()
            //         .contains(&(props.value)().to_lowercase())
            //     {
            //         true
            //     } else {
            //         false
            //     }
            // })
            .collect()
    });

    if let Some(id) = props.id.clone() {
        use_click_outside(
            id,
            move || (props.is_open)(),
            EventHandler::new(move |_| props.is_open.set(false)),
        );
    }

    use_context_provider(|| SearchDropdownContext {
        filtered_results,
        input: props.value,
        is_open: props.is_open,
        selected_options: props
            .selected_options
            .unwrap_or_else(|| Memo::new(Vec::new)),
        selected_option: props.selected_option.unwrap_or_else(|| Signal::new(None)),
        get_label: props.get_label,
    });

    let base_class = "relative w-full";
    rsx!(
        div {
            id: props.id,
            class: format!("{} {}", base_class, props.class.unwrap_or_default()),
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
pub fn SearchDropdownInput<T>(props: SearchDropdownInputProps) -> Element
where
    T: Clone + PartialEq + 'static,
{
    let mut ctx = use_context::<SearchDropdownContext<T>>();
    rsx!(
        Input {
            id: props.id,
            class: props.class,
            placeholder: props.placeholder,
            value: ctx.input,
            width: props.width,
            autofocus: false,
            onclick: move |_| { ctx.is_open.set(true) },
        }
    )
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
pub fn SearchDropdownContent<T>(props: SearchDropdownContentProps) -> Element
where
    T: Clone + PartialEq + 'static,
{
    let mut ctx = use_context::<SearchDropdownContext<T>>();

    let class = format!(
        "absolute z-50 mt-1 max-h-60 overflow-y-auto border-2 border-secondary shadow-lg bg-primary text-secondary {} {}",
        props.class.unwrap_or_default(),
        props.width.unwrap_or("w-full".to_string()),
    );

    if (!(ctx.input)().is_empty() || (ctx.is_open)()) && !ctx.filtered_results.is_empty() {
        rsx!(
            div { id: props.id, class,
                {
                    ctx.filtered_results
                        .iter()
                        .map(|item| {
                            let label = (ctx.get_label)(&item);
                            let item = item.clone();
                            rsx! {
                                button {
                                    class: "flex justify-between items-center gap-2 w-full text-left px-1 py-0.5 hover:bg-primary-1 hover:text-secondary-1",
                                    onclick: move |e: MouseEvent| {
                                        if let Some(handler) = props.onclick {
                                            handler.call(e)
                                        }
                                        ctx.selected_option.set(Some(item.clone()))
                                    },
                                    div { class: "truncate w-full text-sm font-normal", "{label}" }
                                    if (ctx.selected_options)().contains(&item) {
                                        Icon {
                                            height: 16,
                                            width: 16,
                                            icon: FaCheck,
                                            class: "pr-2 w-fit",
                                        }
                                    }
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
