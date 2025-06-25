use dioxus::prelude::*;
use futures_timer::Delay;
use reqwest::Error;
use serde::Serialize;

use super::API;
use crate::views::List;

pub fn get_lists(uuid: String, refetch_signal: Signal<u32>) -> Resource<Option<Vec<List>>> {
    use_resource(move || {
        let _ = refetch_signal();
        let uuid = uuid.clone();
        async move {
            let mut retries = 0;
            loop {
                match reqwest::get(format!("{API}/boards/{uuid}/lists")).await {
                    Ok(res) => match res.json::<Vec<List>>().await {
                        Ok(lists) => break Some(lists),
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
pub struct NewList {
    pub name: String,
    pub board_uuid: String,
    pub description: Option<String>,
    pub position: i32,
}

pub async fn add_list(list: NewList) -> Result<List, Error> {
    let client = reqwest::Client::new();
    let response = client
        .post(format!("{API}/lists"))
        .json(&list)
        .send()
        .await?;
    let list = response.json::<List>().await?;
    tracing::info!("New list has been added: {}", list.uuid);
    Ok(list)
}

pub async fn delete_list(uuid: &str) -> Result<bool, Error> {
    let client = reqwest::Client::new();
    let response = client.delete(format!("{API}/lists/{uuid}")).send().await?;
    if response.status().is_success() {
        Ok(true)
    } else {
        Ok(false)
    }
}

#[derive(Serialize, Clone)]
pub struct UpdateList {
    pub name: Option<String>,
    pub description: Option<String>,
    pub position: Option<i32>,
}

pub async fn update_list(uuid: &str, list: UpdateList) -> Result<bool, Error> {
    let client = reqwest::Client::new();
    let response = client
        .put(format!("{API}/lists/{uuid}"))
        .json(&list)
        .send()
        .await?;
    if response.status().is_success() {
        Ok(true)
    } else {
        Ok(false)
    }
}

pub async fn switch_lists(uuid_from: &str, uuid_to: &str) -> Result<bool, Error> {
    let client = reqwest::Client::new();
    let response = client
        .put(format!("{API}/lists/switch"))
        .json(&(uuid_from, uuid_to))
        .send()
        .await?;
    if response.status().is_success() {
        Ok(true)
    } else {
        Ok(false)
    }
}
