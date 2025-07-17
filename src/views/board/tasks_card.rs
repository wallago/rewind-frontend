use crate::{
    components::{Button, Table, TableBody, TableCaption, TableHead, TableHeader, TableRow},
    context::TasksContext,
    hooks::{use_click_outside, use_tasks_get},
    views::board::{modals::AddTask, task_card::TaskCard},
};
use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::fa_solid_icons::FaPlus};

#[component]
pub fn TasksCard(uuid: String) -> Element {
    use_context_provider(|| TasksContext {
        tasks: Signal::new(Vec::new()),
        refresh: Signal::new(()),
    });

    use_tasks_get(uuid.clone());

    let ctx_tasks = use_context::<TasksContext>();

    let mut is_add_open = use_signal(|| false);

    use_click_outside(
        "add-task-area".to_string(),
        move || is_add_open(),
        EventHandler::new(move |_| is_add_open.set(false)),
    );

    let tasks: Vec<Element> = (ctx_tasks.tasks)()
        .iter()
        .map(|task| rsx!(TaskCard { task: task.clone() }))
        .collect();

    rsx! {
        div {
            Table {
                TableCaption { class: "text-sm", "Tasks" }
                TableHeader { class: "font-semibold text-base text-secondary",
                    TableRow { draggable: false,
                        TableHead { "Tasks" }
                    }
                }
                TableBody { class: "w-full font-medium text-base text-secondary max-h-64",
                    {tasks.iter()}
                }
            }
            Button {
                class: "px-2 justify-between gap-0 font-semibold text-sm ml-auto",
                width: "w-20",
                onclick: move |_| is_add_open.set(true),
                "Task"
                Icon { height: 14, width: 14, icon: FaPlus }
            }
            AddTask { is_open: is_add_open, list_uuid: uuid.clone() }
        }
    }
}
