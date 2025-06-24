use dioxus::prelude::*;
use futures_timer::Delay;
use reqwest::Error;
use serde::Serialize;

use super::API;
use crate::views::Task;

pub fn get_tasks(uuid: String, refetch_signal: Signal<u32>) -> Resource<Option<Vec<Task>>> {
    use_resource(move || {
        let _ = refetch_signal();
        let uuid = uuid.clone();
        async move {
            let mut retries = 0;
            loop {
                match reqwest::get(format!("{API}/lists/{uuid}/tasks")).await {
                    Ok(res) => match res.json::<Vec<Task>>().await {
                        Ok(tasks) => break Some(tasks),
                        Err(_) if retries < 3 => retries += 1,
                        Err(_) => break None,
                    },
                    Err(_) if retries < 3 => retries += 1,
                    Err(_) => break None,
                }

                Delay::new(std::time::Duration::from_secs(2)).await;
            }
        }
    })
}

#[derive(Serialize, Clone)]
pub struct NewTask {
    pub name: String,
    pub description: Option<String>,
    pub list_uuid: String,
    pub position: i32,
    pub status: String,
}

pub async fn add_task(task: NewTask) -> Result<Task, Error> {
    let client = reqwest::Client::new();
    let response = client
        .post(format!("{API}/tasks"))
        .json(&task)
        .send()
        .await?;
    let task = response.json::<Task>().await?;
    tracing::info!("New task has been added: {}", task.uuid);
    Ok(task)
}

pub async fn delete_task(uuid: &str) -> Result<bool, Error> {
    let client = reqwest::Client::new();
    let response = client.delete(format!("{API}/tasks/{uuid}")).send().await?;
    if response.status().is_success() {
        Ok(true)
    } else {
        Ok(false)
    }
}

#[derive(Serialize, Clone)]
pub struct UpdateTask {
    pub name: Option<String>,
    pub description: Option<String>,
    pub position: Option<i32>,
    pub status: Option<String>,
}

pub async fn update_task(uuid: &str, task: UpdateTask) -> Result<bool, Error> {
    let client = reqwest::Client::new();
    let response = client
        .put(format!("{API}/tasks/{uuid}"))
        .json(&task)
        .send()
        .await?;
    if response.status().is_success() {
        Ok(true)
    } else {
        Ok(false)
    }
}
