use dioxus::prelude::*;

#[derive(Clone, Props, PartialEq)]
pub struct BodyProps {
    pub children: Element,
}

#[component]
pub fn Body(props: BodyProps) -> Element {
    rsx!(div {
        class: "mx-[5%] px-4 py-3 my-8 border-2 border-border-light dark:border-border-dark",
        {props.children}
    })
}
