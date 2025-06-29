use dioxus::prelude::*;

#[component]
pub fn Empty(props: super::IconProps) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            height: props.size.clone(),
            view_box: "0 0 24 24",
            width: props.size,
            fill: "currentColor",
            path {
                d: "M12 20A8 8 0 1 1 20 12A8 8 0 0 1 12 20M12 2A10 10 0 1 0 22 12A10 10 0 0 0 12 2Z"
            }
        }
    }
}

#[component]
pub fn Half(props: super::IconProps) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            height: props.size.clone(),
            view_box: "0 0 24 24",
            width: props.size,
            fill: "currentColor",
            path {
                d: "M12 2A10 10 0 0 0 2 12A10 10 0 0 0 12 22A10 10 0 0 0 22 12A10 10 0 0 0 12 2M12 4A8 8 0 0 1 20 12A8 8 0 0 1 12 20V4Z"
            }
        }

    }
}

#[component]
pub fn Full(props: super::IconProps) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            height: props.size.clone(),
            view_box: "0 0 24 24",
            width: props.size,
            fill: "currentColor",
            path {
                d: "M12 2A10 10 0 1 1 2 12A10 10 0 0 1 12 2Z"
            }
        }
    }
}
