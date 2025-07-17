use dioxus::prelude::*;

use crate::models::Task;

#[derive(Clone)]
pub struct TasksContext {
    pub tasks: Signal<Vec<Task>>,
    pub refresh: Signal<()>,
}
