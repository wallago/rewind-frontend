use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct TableProps {
    children: Element,
    #[props(optional)]
    class: Option<String>,
    #[props(optional)]
    colspan: Option<u32>,
    #[props(optional)]
    onclick: Option<EventHandler<MouseEvent>>,
}

#[component]
pub fn Table(props: TableProps) -> Element {
    rsx!(
        div {
            class: "overflow-auto",
            table { class: "w-full caption-bottom text-sm",
                {props.children}
            }
        }
    )
}

#[component]
pub fn TableCaption(props: TableProps) -> Element {
    rsx!(
        caption {
            class: format!("mt-4 text-sm text-text-light/60 dark:text-text-dark/50 {}", props.class.unwrap_or_default()),
            {props.children}
        }
    )
}

#[component]
pub fn TableHeader(props: TableProps) -> Element {
    rsx!(
        thead {
            class: format!("font-semibold {}",props.class.unwrap_or_default()),
            {props.children}
        }
    )
}

#[component]
pub fn TableRow(props: TableProps) -> Element {
    rsx!(
        tr {
            onclick: move |e: MouseEvent| {
                e.stop_propagation();
                if let Some(handler) = &props.onclick {
                    handler.call(e);
                }
            },

            class: format!("
                       border-b-2 border-border-light dark:border-border-dark 
                       {}", props.class.unwrap_or_default()),
            {props.children}
        }
    )
}

#[component]
pub fn TableHead(props: TableProps) -> Element {
    rsx!(
        th {
            class: format!("h-12 px-4 text-left align-middle {}", props.class.unwrap_or_default()),
            colspan: props.colspan.unwrap_or(1),
            {props.children}
        }
    )
}

#[component]
pub fn TableBody(props: TableProps) -> Element {
    rsx!(
        tbody {
            class: props.class.unwrap_or_default(),
            {props.children}
        }
    )
}

#[component]
pub fn TableCell(props: TableProps) -> Element {
    rsx!(
        td {
            class: format!("p-4 align-middle {}", props.class.unwrap_or_default()),
            colspan: props.colspan.unwrap_or(1),
            {props.children}
        }
    )
}

#[component]
pub fn TableFooter(props: TableProps) -> Element {
    rsx!(
        tfoot {
            class: format!("font-medium bg-muted-light dark:bg-muted-light {}", props.class.unwrap_or_default()),
            {props.children}
        }
    )
}
