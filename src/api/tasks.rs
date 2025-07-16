use reqwest::{Error, StatusCode};

use crate::{
    api::BASE_URL,
    models::{NewTask, Task, UpdateTask},
};

pub async fn get_tasks_by_list_uuid(list_uuid: String) -> Result<Vec<Task>, Error> {
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{BASE_URL}/lists/{list_uuid}/tasks"))
        .send()
        .await?;
    let tasks = response.json::<Vec<Task>>().await?;
    Ok(tasks)
}

pub async fn add_task(task: NewTask) -> Result<Task, Error> {
    let client = reqwest::Client::new();
    let response = client
        .post(format!("{BASE_URL}/tasks"))
        .json(&task)
        .send()
        .await?;
    let task = response.json::<Task>().await?;
    Ok(task)
}

pub async fn delete_task(uuid: &str) -> Result<StatusCode, Error> {
    let client = reqwest::Client::new();
    let response = client
        .delete(format!("{BASE_URL}/tasks/{uuid}"))
        .send()
        .await?;
    Ok(response.status())
}

pub async fn update_task(uuid: &str, task: UpdateTask) -> Result<StatusCode, Error> {
    let client = reqwest::Client::new();
    let response = client
        .put(format!("{BASE_URL}/tasks/{uuid}"))
        .json(&task)
        .send()
        .await?;
    Ok(response.status())
}

pub async fn switch_tasks(uuid_from: &str, uuid_to: &str) -> Result<StatusCode, Error> {
    let client = reqwest::Client::new();
    let response = client
        .put(format!("{BASE_URL}/tasks/switch"))
        .json(&(uuid_from, uuid_to))
        .send()
        .await?;
    Ok(response.status())
}
