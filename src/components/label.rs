use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct LabelProps {
    pub children: Element,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub r#for: Option<String>,
    #[props(optional)]
    pub variant: Option<String>,
    #[props(optional)]
    pub width: Option<String>,
}

#[component]
pub fn Label(props: LabelProps) -> Element {
    let variant_class = match props.variant.as_deref() {
        Some("outline") => "bg-primary text-secondary border-2 border-secondary",
        Some("title") => "bg-primary text-secondary font-semibold",
        Some("title_1") => "bg-primary-1 text-secondary font-semibold",
        _ => "bg-secondary text-primary",
    };
    let base_class = "
        p-1 flex items-center leading-none inline-block max-w-full 
        peer-disabled:cursor-not-allowed peer-disabled:opacity-70
    ";

    rsx!(
        label {
            class: format!(
                "{} {} {} {}",
                base_class,
                variant_class,
                props.class.clone().unwrap_or_default(),
                props.width.clone().unwrap_or("w-fit".to_string()),
            ),
            r#for: props.r#for.clone().unwrap_or_default(),
            {props.children}
        }
    )
}
