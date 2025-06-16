use dioxus::prelude::*;

#[component]
pub fn Contrast() -> Element {
    rsx! {
    svg {
        xmlns: "http://www.w3.org/2000/svg",
        view_box: "0 0 24 24",
        fill: "none",
        stroke: "currentColor",
        stroke_width: "2",
        stroke_linecap: "round",
        stroke_linejoin: "round",
        path {
            stroke: "none",
            d: "M0 0h24v24H0z",
            fill: "none"
        }
        line {
            x1: "4",
            y1: "3",
            x2: "21",
            y2: "3",
        }
        line {
            x1: "3",
            y1: "21",
            x2: "21",
            y2: "21",
        }
        line {
            x1: "3",
            y1: "3",
            x2: "3",
            y2: "21",
        }
        line {
            x1: "21",
            y1: "3",
            x2: "21",
            y2: "21",
        }
        line {
            x1: "12",
            y1: "3",
            x2: "12",
            y2: "21"
        }
        line {
            x1: "12",
            y1: "9",
            x2: "17.81",
            y2: "3.19",
        }

        line {
            x1: "12",
            y1: "14.3",
            x2: "21.0",
            y2: "5.0"
        }
        line {
            x1: "12",
            y1: "19.6",
            x2: "20.85",
            y2: "10.75"
        }
        line {
            x1: "15.5",
            y1: "21",
            x2: "20.85",
            y2: "16"
        }
    }
    }
}
