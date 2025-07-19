use crate::{
    components::{
        Accordion, AccordionContent, AccordionTrigger, Button, Dialog, DialogClose, DialogContent,
        DialogFooter, DialogHeader, DialogTitle, Input, Label, RadioGroup, RadioGroupItem,
        SearchDropdown, SearchDropdownContent, SearchDropdownInput, Textarea,
    },
    context::{TagsContext, TaskTagsContext},
    hooks::{use_click_outside, use_task_tag_link, use_task_tag_unlink, use_task_update},
    models::{Priority, Status, Tag, Task},
    views::board::{
        add_tag_modal::AddTag, delete_task_modal::DeleteTask, update_tags_modal::UpdateTags,
    },
};
use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::fa_solid_icons::{FaGear, FaPlus},
};

#[component]
pub fn UpdateTask(is_open: Signal<bool>, task: Task) -> Element {
    let name = use_signal(|| task.name.clone());
    let status = use_signal(|| Some(task.status.clone()));
    let priority = use_signal(|| Some(task.priority.clone()));

    let tag_search = use_signal(|| "".to_string());

    let mut is_delete_open = use_signal(|| false);
    let mut is_add_tag_open = use_signal(|| false);
    let mut is_update_tags_open = use_signal(|| false);
    let is_desc_open = use_signal(|| false);
    let is_search_tags_open = use_signal(|| false);

    let mut trigger_update = use_signal(|| false);
    use_task_update(name, status, priority, task.uuid.clone(), trigger_update);

    let ctx_tags = use_context::<TagsContext>();
    let ctx_task_tags = use_context::<TaskTagsContext>();
    let task_tags = ctx_task_tags.task_tags.peek().clone();
    let tags = use_memo(move || (ctx_tags.tags)().clone());
    let selected_tags = use_memo(move || (ctx_task_tags.task_tags)().clone());

    use_click_outside(
        "update-task-area".to_string(),
        move || is_open(),
        EventHandler::new(move |_| {
            if !is_add_tag_open()
                && !is_delete_open()
                && !is_search_tags_open()
                && !is_update_tags_open()
            {
                is_open.set(false)
            }
        }),
    );

    let selected_tag = use_signal(|| None::<Tag>);

    let mut trigger_link = use_signal(|| false);
    let mut trigger_unlink = use_signal(|| false);
    let task_tags_copy = task_tags.clone();
    use_effect(move || {
        if let Some(tag) = selected_tag() {
            if task_tags_copy.contains(&tag) {
                trigger_unlink.set(true);
            } else {
                trigger_link.set(true);
            }
        }
    });

    use_task_tag_link(
        task.uuid.clone(),
        match selected_tag() {
            Some(tag) => tag.uuid,
            _ => Default::default(),
        },
        trigger_link,
    );
    use_task_tag_unlink(
        task.uuid.clone(),
        match selected_tag() {
            Some(tag) => tag.uuid,
            _ => Default::default(),
        },
        trigger_unlink,
    );

    rsx! {
        Dialog { is_open,
            DialogContent { id: "update-task-area", class: "sm:max-w-[425px] h-fit",
                DialogHeader {
                    div { class: "flex justify-between items-center",
                        DialogTitle { "Update {task.name}" }
                        Button {
                            onclick: move |_| {
                                is_delete_open.set(true);
                            },
                            class: "px-2 text-xs h-6",
                            "Delete"
                        }
                    }
                }
                div { class: "pb-0.5" }
                Input {
                    label: "name:",
                    width: "w-full",
                    placeholder: "Enter list name",
                    value: name,
                    onenter: EventHandler::new(move |_e: KeyboardEvent| {
                        trigger_update.set(true);
                        is_open.set(false);
                    }),
                }
                div { class: "flex items-stretch gap-2 justify-between pt-2",
                    RadioGroup::<Status> { class: "pt-2 w-1/3", selected: status,
                        RadioGroupItem::<Status> {
                            value: Status::Todo,
                            label: Status::Todo.to_string(),
                        }
                        RadioGroupItem::<Status> {
                            value: Status::InProgress,
                            label: Status::InProgress.to_string(),
                        }
                        RadioGroupItem::<Status> {
                            value: Status::Done,
                            label: Status::Done.to_string(),
                        }
                    }
                    div { class: "w-1 my-1 bg-secondary" }
                    RadioGroup::<Priority> { class: "pt-2 w-1/3", selected: priority,
                        RadioGroupItem::<Priority> {
                            value: Priority::Low,
                            label: Priority::Low.to_string(),
                        }
                        RadioGroupItem::<Priority> {
                            value: Priority::Medium,
                            label: Priority::Medium.to_string(),
                        }
                        RadioGroupItem::<Priority> {
                            value: Priority::High,
                            label: Priority::High.to_string(),
                        }
                    }
                }
                Accordion { class: "pt-2", is_open: is_desc_open.clone(),
                    AccordionTrigger { class: "text-sm", label: "description" }
                    AccordionContent { class: "text-sm",
                        Textarea {
                            width: "w-full",
                            value: use_signal(|| "lsdkfjlsdf".to_string()),
                        }
                    }
                }
                Label {
                    variant: "title",
                    class: "p-2 bg-primary",
                    width: "w-full",
                    "Tags"
                }
                div { class: "flex justify-between gap-2",
                    SearchDropdown::<Tag> {
                        id: "search-tags-area",
                        options: tags,
                        is_open: is_search_tags_open,
                        value: tag_search,
                        class: "text-base",
                        selected_options: selected_tags,
                        selected_option: selected_tag,
                        get_label: |tag: &Tag| tag.name.clone(),
                        SearchDropdownInput::<Tag> { width: "w-full", placeholder: " Search tags" }
                        SearchDropdownContent::<Tag> {}
                    }
                    Button {
                        class: "px-2 justify-between gap-2 font-semibold text-base",
                        onclick: move |_| is_update_tags_open.set(true),
                        Icon { height: 14, width: 14, icon: FaGear }
                    }
                    Button {
                        class: "px-2 justify-between gap-2 font-semibold text-base",
                        onclick: move |_| is_add_tag_open.set(true),
                        Icon { height: 14, width: 14, icon: FaPlus }
                    }
                }
                DialogFooter {
                    DialogClose {}
                    Button {
                        onclick: move |_| {
                            trigger_update.set(true);
                            is_open.set(false);
                        },
                        r#type: "submit",
                        variant: "outline",
                        class: "font-semibold px-2 text-sm",
                        "Save"
                    }
                }
            }
        }
        DeleteTask {
            is_open: is_delete_open,
            task: task.clone(),
            parent_open: is_open,
        }
        AddTag { is_open: is_add_tag_open }
        UpdateTags { is_open: is_update_tags_open }
    }
}
