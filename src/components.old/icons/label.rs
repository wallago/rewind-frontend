use dioxus::prelude::*;

#[component]
pub fn Label(props: super::IconProps) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            height: props.size.clone(),
            view_box: "0 0 24 24",
            width: props.size,
            fill: "currentColor",
            path {
                d: "m3.5 18.99l11 .01c.67 0 1.27-.33 1.63-.84L20.5 12l-4.37-6.16c-.36-.51-.96-.84-1.63-.84l-11 .01L8.34 12L3.5 18.99z"
            }
        }
    }
}
