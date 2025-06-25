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
                d: "M12 20a8 8 0 1 1 8-8a8 8 0 0 1-8 8m0-18a10 10 0 1 0 10 10A10 10 0 0 0 12 2Z"
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
                d: "M512 1024q-104 0-199-40.5t-163.5-109T40.5 711T0 512t40.5-199t109-163.5T313 40.5T512 0t199 40.5t163.5 109t109 163.5t40.5 199t-40.5 199t-109 163.5t-163.5 109t-199 40.5zm0-960q-91 0-174 35.5T195 195T99.5 338T64 512t35.5 174T195 829t143 95.5T512 960V64z"
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
                d: "M512 0Q408 0 313 40.5t-163.5 109T40.5 313T0 512t40.5 199t109 163.5t163.5 109t199 40.5t199-40.5t163.5-109t109-163.5t40.5-199t-40.5-199t-109-163.5T711 40.5T512 0z"
            }
        }
    }
}
