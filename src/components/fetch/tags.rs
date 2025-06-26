use dioxus::prelude::*;
use futures_timer::Delay;
use reqwest::Error;

use super::API;
use crate::components::models::{NewTag, Tag, UpdateTag};

// pub fn get_tasks(uuid: String, refetch_signal: Signal<u32>) -> Resource<Option<Vec<Task>>> {
//     use_resource(move || {
//         let _ = refetch_signal();
//         let uuid = uuid.clone();
//         async move {
//             let mut retries = 0;
//             loop {
//                 match reqwest::get(format!("{API}/lists/{uuid}/tasks")).await {
//                     Ok(res) => match res.json::<Vec<Task>>().await {
//                         Ok(tasks) => break Some(tasks),
//                         Err(_) if retries < 3 => retries += 1,
//                         Err(_) => break None,
//                     },
//                     Err(_) if retries < 3 => retries += 1,
//                     Err(_) => break None,
//                 }

//                 Delay::new(std::time::Duration::from_secs(2)).await;
//             }
//         }
//     })
// }

pub async fn add_tag(tag: NewTag) -> Result<Tag, Error> {
    let client = reqwest::Client::new();
    let response = client.post(format!("{API}/tags")).json(&tag).send().await?;
    let tag = response.json::<Tag>().await?;
    tracing::info!("New tag has been added: {}", tag.uuid);
    Ok(tag)
}

// pub async fn delete_task(uuid: &str) -> Result<bool, Error> {
//     let client = reqwest::Client::new();
//     let response = client.delete(format!("{API}/tasks/{uuid}")).send().await?;
//     if response.status().is_success() {
//         Ok(true)
//     } else {
//         Ok(false)
//     }
// }

// pub async fn update_task(uuid: &str, task: UpdateTask) -> Result<bool, Error> {
//     let client = reqwest::Client::new();
//     let response = client
//         .put(format!("{API}/tasks/{uuid}"))
//         .json(&task)
//         .send()
//         .await?;
//     if response.status().is_success() {
//         Ok(true)
//     } else {
//         Ok(false)
//     }
// }
