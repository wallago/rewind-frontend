use dioxus::prelude::*;

#[component]
pub fn Cross(props: super::IconProps) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            height: props.size.clone(),
            view_box: "0 -960 960 960",
            width: props.size,
            path {
                d: "m256-200-56-56 224-224-224-224 56-56 224 224 224-224 56 56-224 224 224 224-56 56-224-224-224 224Z"
            }
        }
    }
}
