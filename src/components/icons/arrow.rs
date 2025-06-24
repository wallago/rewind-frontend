use dioxus::prelude::*;

#[component]
pub fn Bottom(props: super::IconProps) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            height: props.size.clone(),
            view_box: "0 -960 960 960",
            width: props.size,
            path {
                d: "M480-344 240-584l56-56 184 184 184-184 56 56-240 240Z"
            }
        }
    }
}
