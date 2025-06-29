use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct LabelProps {
    pub children: Element,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub r#for: Option<String>,
}

#[component]
pub fn Label(props: LabelProps) -> Element {
    let base_class = "
        text-sm font-semibold leading-none
        peer-disabled:cursor-not-allowed peer-disabled:opacity-70
    ";

    rsx!(
        label {
            class: format!(
                "{} {}",
                base_class,
                props.class.clone().unwrap_or_default()
            ),
            r#for: props.r#for.clone().unwrap_or_default(),
            {props.children}
        }
    )
}
