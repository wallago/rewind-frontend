use crate::components::Button;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_solid_icons::FaCheck;

#[derive(Clone)]
struct RadioGroupContext<T>
where
    T: Clone + PartialEq + 'static,
{
    selected: Signal<Option<T>>,
    items: Vec<T>,
}

#[derive(PartialEq, Clone, Props)]
pub struct RadioGroupProps<T>
where
    T: Clone + PartialEq + 'static,
{
    pub selected: Signal<Option<T>>,
    pub children: Element,
    #[props(optional)]
    pub class: Option<String>,
}

#[component]
pub fn RadioGroup<T>(props: RadioGroupProps<T>) -> Element
where
    T: Clone + PartialEq + 'static,
{
    use_context_provider(|| RadioGroupContext {
        selected: props.selected,
        items: Vec::<T>::new(),
    });

    let base_class = "flex flex-col gap-2";
    let class = format!("{} {}", base_class, props.class.unwrap_or_default());
    rsx!(
        div {
            class,
            {props.children}
        }
    )
}

#[derive(PartialEq, Clone, Props)]
pub struct RadioGroupItemProps<T>
where
    T: Clone + PartialEq + 'static,
{
    pub label: String,
    pub value: T,
    #[props(optional)]
    pub class: Option<String>,
}

#[component]
pub fn RadioGroupItem<T>(props: RadioGroupItemProps<T>) -> Element
where
    T: Clone + PartialEq + 'static,
{
    let mut ctx = use_context::<RadioGroupContext<T>>();

    let is_selected = (ctx.selected)() == Some(props.value.clone());
    let value = props.value.clone();
    use_effect(move || {
        if !ctx.items.contains(&value) {
            ctx.items.push(value.clone());
        }
    });

    let base_class = "flex gap-4";
    let class = format!("{} {}", base_class, props.class.unwrap_or_default());

    rsx!(
        div {
            class,
            Button {
                variant: "outline",
                onclick: move |_| {
                    ctx.selected.set(Some(props.value.clone()));
                },
                if is_selected {
                    Icon { height: 16, icon: FaCheck }
                } else {
                    Icon { height: 16, icon: FaCheck , class: "text-transparent" }
                }
            }
            span { "{props.label}" }
        }
    )
}
