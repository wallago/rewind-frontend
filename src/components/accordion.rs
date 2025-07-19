use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_solid_icons::FaChevronDown;

#[derive(Clone)]
struct AccordionContext {
    is_open: Signal<bool>,
}

#[derive(PartialEq, Clone, Props)]
pub struct AccordionProps {
    pub is_open: Signal<bool>,
    #[props(optional)]
    pub class: Option<String>,
    pub children: Element,
}

#[component]
pub fn Accordion(props: AccordionProps) -> Element {
    use_context_provider(|| AccordionContext {
        is_open: props.is_open,
    });

    let base_class = "relative text-base text-secondary w-full flex flex-col gap-2";
    let class = format!("{} {}", base_class, props.class.unwrap_or_default());
    rsx! {
        div { class, {props.children} }
    }
}

#[derive(PartialEq, Clone, Props)]
pub struct AccordionContentProps {
    #[props(optional)]
    pub class: Option<String>,
    pub children: Element,
}

#[component]
pub fn AccordionContent(props: AccordionContentProps) -> Element {
    let ctx = use_context::<AccordionContext>();

    let base_class = "w-full";
    let class = format!("{} {}", base_class, props.class.unwrap_or_default());
    rsx! {
        if (ctx.is_open)() {
            div { class: "w-full mx-auto h-0.5 bg-secondary" }
            div { class, {props.children} }
        }
    }
}

#[derive(PartialEq, Clone, Props)]
pub struct AccordionTriggerProps {
    pub label: String,
    #[props(optional)]
    pub class: Option<String>,
}

#[component]
pub fn AccordionTrigger(props: AccordionTriggerProps) -> Element {
    let mut ctx = use_context::<AccordionContext>();

    let base_class = "w-full";
    let class = format!("{} {}", base_class, props.class.unwrap_or_default());
    let icon_class = if (ctx.is_open)() {
        "transform rotate-180 transition-transform duration-200"
    } else {
        "transform rotate-0 transition-transform duration-200"
    };
    rsx! {
        button { class, onclick: move |_| ctx.is_open.toggle(),
            div { class: "flex justify-between items-center",
                "{props.label}"
                Icon { height: 14, icon: FaChevronDown, class: icon_class }
            }
        }
    }
}
