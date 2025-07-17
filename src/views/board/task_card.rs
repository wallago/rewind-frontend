use crate::{
    api::get_tags_by_task_uuid,
    components::TableRow,
    models::{Priority, Status, Task},
    views::board::modals::UpdateTask,
};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TaskCardProps {
    task: Task,
}

#[component]
pub fn TaskCard(props: TaskCardProps) -> Element {
    let mut is_update_open = use_signal(|| false);
    let uuid = props.task.uuid.clone();

    let tags = use_resource(move || {
        let uuid = uuid.clone();
        async move {
            match get_tags_by_task_uuid(&uuid).await {
                Ok(fetched) => Some(fetched),
                Err(err) => {
                    tracing::error!("{err}");
                    None
                }
            }
        }
    });

    rsx! {
        div {
            TableRow {
                class: "cursor-pointer",
                onclick: move |_| {
                    is_update_open.set(true)
                },
                div { class: "flex items-center gap-2",
                    div { class: "w-full flex items-center h-full gap-4",
                        {<Priority as Into<Element>>::into(props.task.priority.clone())}
                        {<Status as Into<Element>>::into(props.task.status.clone())}
                        div { class: "flex-grow flex-shrink inline-block truncate", {props.task.name.clone()} }
                    }
                }
                div {
                    class:"flex py-0.5",
                    {
                        match tags() {
                            Some(Some(tags)) => rsx! {
                                if !tags.is_empty() {
                                    {tags.iter().map(|tag| rsx!{
                                       div {
                                           style: format!("--tag-color: {};", tag.color),
                                           class: format!("bg-[var(--tag-color)] w-8 h-1.5 rounded-full"),
                                       },
                                    })}
                                }
                            },
                            _ => rsx! {
                                "Loading tags"
                            },
                        }
                    }
                }
            }
            UpdateTask { is_open: is_update_open , task: props.task.clone() }
        }
    }
}
