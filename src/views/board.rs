use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::fa_regular_icons::FaCircle as FaCircleEmpty,
    icons::fa_solid_icons::{FaChevronRight, FaCircle, FaCircleHalfStroke, FaTag},
};

use crate::components::{
    Label, Table, TableBody, TableCaption, TableCell, TableFooter, TableHead, TableHeader, TableRow,
};

#[component]
pub fn Board(uuid: String) -> Element {
    rsx! {
        div {
            class: "p-4 h-full bg-primary border-2 border-secondary flex flex-col gap-4",
            Header { uuid }
            div {
                class: "grid gap-4 grid-cols-[repeat(auto-fit,_minmax(26rem,_1fr))]",
                List {}
                List {}
                List {}
            }
        }
    }
}

#[component]
fn Header(uuid: String) -> Element {
    rsx! {
        div {
            class: "flex gap-4 items-center",
            Label {
                "Boards"
            }
            Icon {
                class: "text-secondary",
                height: 20,
                icon: FaChevronRight,
            }
            Label {
                {uuid}
            }
        }
    }
}

#[component]
fn List() -> Element {
    rsx! {
        div {
            class: "h-fit w-96 p-2 bg-primary-2 border-2 border-secondary flex flex-col gap-2",
            div {
                class: "flex text-sm font-medium gap-2 w-full",
                Label {
                    variant: "outline",
                    class: "p-2",
                    width: "w-1/2",
                    div {
                        class: "truncate",
                        "Name: Backend"
                    }
                }
                Label {
                    variant: "outline",
                    class: "p-2",
                    width: "w-1/2",
                    div {
                        class: "truncate",
                        "UUID: 571a9fa0-1bb4-4545-bdd3-b7315dcb6615"
                    }
                }
            }
            Tasks {}
        }
    }
}

#[component]
fn Tasks() -> Element {
    rsx! {
        div {
            Table {
                TableCaption {
                    class: "text-xs",
                    "Tasks"
                }
                TableHeader {
                    class: "font-semibold text-sm text-secondary",
                    TableRow {
                        TableHead {
                            "Tasks"
                        }
                    }
                }
                TableBody {
                    class: "w-full font-medium text-sm text-secondary h-64",
                    TaskDemo1 {  }
                    TaskDemo2 {  }
                    TaskDemo3 {  }
                    TaskDemo1 {  }
                    TaskDemo2 {  }
                    TaskDemo3 {  }
                    TaskDemo1 {  }
                    TaskDemo2 {  }
                    TaskDemo3 {  }
                    TaskDemo1 {  }
                    TaskDemo2 {  }
                    TaskDemo3 {  }
                    TaskDemo1 {  }
                    TaskDemo2 {  }
                    TaskDemo3 {  }
                    TaskDemo1 {  }
                    TaskDemo2 {  }
                    TaskDemo3 {  }
                }
            }
        }
    }
}

#[component]
fn TaskDemo1() -> Element {
    rsx! {
        TableRow {
            TableCell {
                class: "w-fit pr-2",
                Icon {
                    class: "text-priority-low",
                    height: 16,
                    icon: FaTag,
                }
            }
            TableCell {
                class: "w-full truncate",
                "Websocker with Actix"
            }
            TableCell {
                class: "w-fit pl-2",
                Icon {
                    class: "text-secondary",
                    height: 16,
                    icon: FaCircleHalfStroke,
                }
            }
        }
    }
}

#[component]
fn TaskDemo2() -> Element {
    rsx! {
        TableRow {
            TableCell {
                class: "w-fit pr-2",
                Icon {
                    class: "text-priority-medium",
                    height: 16,
                    icon: FaTag,
                }
            }
            TableCell {
                class: "w-full truncate",
                "Websocker with Actix"
            }
            TableCell {
                class: "w-fit pl-2",
                Icon {
                    class: "text-secondary",
                    height: 16,
                    icon: FaCircleEmpty,
                }
            }
        }
    }
}

#[component]
fn TaskDemo3() -> Element {
    rsx! {
        TableRow {
            TableCell {
                class: "w-fit pr-2",
                Icon {
                    class: "text-priority-high",
                    height: 16,
                    icon: FaTag,
                }
            }
            TableCell {
                class: "w-full truncate",
                "Websocker with Actix"
            }
            TableCell {
                class: "w-fit pl-2",
                Icon {
                    class: "text-secondary",
                    height: 16,
                    icon: FaCircle,
                }
            }
        }
    }
}
