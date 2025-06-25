use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct TableProps {
    children: Element,
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

#[derive(PartialEq, Props, Clone)]
pub struct TableCaptionProps {
    children: Element,
    #[props(optional)]
    class: Option<String>,
}

#[component]
pub fn TableCaption(props: TableCaptionProps) -> Element {
    rsx!(
        caption {
            class: format!("mt-4 text-sm text-text-light/60 dark:text-text-dark/50 {}", props.class.unwrap_or_default()),
            {props.children}
        }
    )
}

#[derive(PartialEq, Props, Clone)]
pub struct TableHeaderProps {
    children: Element,
    #[props(optional)]
    class: Option<String>,
}

#[component]
pub fn TableHeader(props: TableHeaderProps) -> Element {
    rsx!(
        thead {
            class: format!("font-semibold {}",props.class.unwrap_or_default()),
            {props.children}
        }
    )
}

#[derive(PartialEq, Props, Clone)]
pub struct TableRowProps {
    children: Element,
    #[props(optional)]
    class: Option<String>,
    #[props(optional)]
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(optional)]
    ondragstart: Option<EventHandler<DragEvent>>,
    #[props(optional)]
    ondragover: Option<EventHandler<DragEvent>>,
    #[props(optional)]
    ondrop: Option<EventHandler<DragEvent>>,
    #[props(optional)]
    draggable: Option<bool>,
}

#[component]
pub fn TableRow(props: TableRowProps) -> Element {
    rsx!(
        tr {
            draggable: true,
            ondragstart: move |e| {
                if let Some(handler) = &props.ondragstart {
                    handler.call(e);
                }
            },
            ondragover: move |e| {
                e.prevent_default();
                if let Some(handler) = &props.ondragover {
                    handler.call(e);
                }
            },
            ondrop: move |e| {
                if let Some(handler) = &props.ondrop {
                    handler.call(e);
                }
            },
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

#[derive(PartialEq, Props, Clone)]
pub struct TableHeadProps {
    children: Element,
    #[props(optional)]
    class: Option<String>,
    #[props(optional)]
    colspan: Option<u32>,
}

#[component]
pub fn TableHead(props: TableHeadProps) -> Element {
    rsx!(
        th {
            class: format!("h-12 px-4 text-left align-middle {}", props.class.unwrap_or_default()),
            colspan: props.colspan.unwrap_or(1),
            {props.children}
        }
    )
}

#[derive(PartialEq, Props, Clone)]
pub struct TableBodyProps {
    children: Element,
    #[props(optional)]
    class: Option<String>,
}

#[component]
pub fn TableBody(props: TableBodyProps) -> Element {
    rsx!(
        tbody {
            class: props.class.unwrap_or_default(),
            {props.children}
        }
    )
}

#[derive(PartialEq, Props, Clone)]
pub struct TableCellProps {
    children: Element,
    #[props(optional)]
    class: Option<String>,
    #[props(optional)]
    colspan: Option<u32>,
}

#[component]
pub fn TableCell(props: TableCellProps) -> Element {
    rsx!(
        td {
            class: format!("p-4 align-middle {}", props.class.unwrap_or_default()),
            colspan: props.colspan.unwrap_or(1),
            {props.children}
        }
    )
}

#[derive(PartialEq, Props, Clone)]
pub struct TableFooterProps {
    children: Element,
    #[props(optional)]
    class: Option<String>,
}

#[component]
pub fn TableFooter(props: TableFooterProps) -> Element {
    rsx!(
        tfoot {
            class: format!("font-medium bg-muted-light dark:bg-muted-light {}", props.class.unwrap_or_default()),
            {props.children}
        }
    )
}
