use crate::{
    components::TableRow,
    context::{TagsContext, TaskTagsContext},
    hooks::use_task_tags_get,
    models::{Priority, Status, Task},
    views::board::update_task_modal::UpdateTask,
};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TaskCardProps {
    task: Task,
}

#[component]
pub fn TaskCard(props: TaskCardProps) -> Element {
    use_context_provider(|| TaskTagsContext {
        task_tags: Signal::new(Vec::new()),
        refresh: Signal::new(()),
    });

    use_task_tags_get(props.task.uuid.clone());

    let mut is_update_open = use_signal(|| false);

    let mut ctx_task_tags = use_context::<TaskTagsContext>();
    let tags = ctx_task_tags.task_tags;

    let ctx_tags = use_context::<TagsContext>();

    use_effect(move || {
        (ctx_tags.refresh)();
        ctx_task_tags.refresh.set(());
    });

    rsx! {
        div {
            TableRow {
                class: "cursor-pointer",
                onclick: move |_| { is_update_open.set(true) },
                div { class: "flex items-center gap-2",
                    div { class: "w-full flex items-center h-full gap-4",
                    div { class: "flex-none", {<Priority as Into<Element>>::into(props.task.priority.clone())} }
                    div { class: "flex-none", {<Status as Into<Element>>::into(props.task.status.clone())} }
                        div { class: "flex-grow flex-shrink inline-block truncate",
                            {props.task.name.clone()}
                        }
                    }
                }
                div { class: "flex py-0.5 gap-0.5",
                    if !tags.is_empty() {
                        {tags.iter().map(|tag| rsx! {
                            div {
                                style: format!("--tag-color: {};", tag.color),
                                class: format!("bg-[var(--tag-color)] w-8 h-1.5 rounded-full"),
                            }
                        })}
                    }
                }
            }
            UpdateTask { is_open: is_update_open, task: props.task.clone() }
        }
    }
}
