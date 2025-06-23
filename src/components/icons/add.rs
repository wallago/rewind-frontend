use dioxus::prelude::*;

#[component]
pub fn Add(props: super::IconProps) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            height: props.size.clone(),
            view_box: "0 -960 960 960",
            width: props.size,
            class: "fill-text-light dark:fill-text-dark",
            path {
                d: "M440-120v-320H120v-80h320v-320h80v320h320v80H520v320h-80Z"
            }
        }
    }
}
