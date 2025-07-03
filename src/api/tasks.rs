use reqwest::Error;

use crate::{
    api::BASE_URL,
    models::{NewTask, Task},
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
