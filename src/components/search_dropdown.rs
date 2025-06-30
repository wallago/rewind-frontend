use dioxus::prelude::*;

use crate::components::Input;

#[derive(PartialEq, Clone, Props)]
pub struct SearchDropdownProps {
    pub value: Signal<String>,
    #[props(optional)]
    pub placeholder: Option<String>,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub id: Option<String>,
    #[props(optional)]
    pub width: Option<String>,
    #[props(optional)]
    pub onchange: Option<EventHandler<FormEvent>>,
    pub children: Element,
}

#[component]
pub fn SearchDropdown(props: SearchDropdownProps) -> Element {
    let querry_results = ["result 1".to_string(), "result 2".to_string()].to_vec();

    let filtered_results: Vec<Element> = querry_results
        .into_iter()
        .filter(|result| 
            result.to_lowercase().contains(&(props.value)().to_lowercase())
        )
        .map(|result| {
            rsx!(
                div {
                   class: "flex flex-col gap-2 p-1 hover:bg-primary-2 hovertext-secondary-2 cursor-pointer text-sm",
                    {result}
                }
            )
        })
        .collect();

    let base_class = "relative w-full";
    rsx!(
        div {
            class: base_class,
            id: props.id,
            Input {
                class: props.class,
                placeholder: props.placeholder,
                value: props.value,
                width: props.width,
            }
            if !(props.value)().is_empty() && !filtered_results.is_empty() {
                div {
                class: "
                    absolute z-50 mt-1 w-full max-h-60 overflow-y-auto
                    border-2 border-secondary shadow-lg
                    bg-primary text-secondary
                    ",
                    {filtered_results.iter()}
                }
            }
        }
    )
}
