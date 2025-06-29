use dioxus::prelude::*;

const CHECKBOX_CSS: Asset = asset!("/assets/styling/checkbox.css");

#[derive(Props, PartialEq, Clone)]
pub struct CheckboxProps {
    pub checked: bool,
    pub on_change: EventHandler<bool>,
    #[props(optional)]
    pub label: Option<Element>,
}

#[component]
pub fn Checkbox(props: CheckboxProps) -> Element {
    rsx!(
        document::Link { rel: "stylesheet", href: CHECKBOX_CSS }
        div {
            class: "flex gap-1",
            {props.label.clone()}
            label {
                class: "flex items-center gap-2 cursor-pointer",
                input {
                    r#type: "checkbox",
                    class: "hidden peer",
                    checked: props.checked,
                    oninput: move |event| {
                        let is_enabled = event.value() == "true";
                        props.on_change.call(is_enabled)
                    }
                }
                span {
                    class: "custom-checkbox w-4 h-4 border-2 text-border-dark dark:border-border-dark border-border-light  transition-colors",
                }
            }
        }
    )
}
