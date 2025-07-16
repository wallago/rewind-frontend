use reqwest::{Error, StatusCode};

use crate::{
    api::BASE_URL,
    models::{List, NewList, UpdateList},
};

pub async fn get_lists_by_board_uuid(board_uuid: String) -> Result<Vec<List>, Error> {
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{BASE_URL}/boards/{board_uuid}/lists"))
        .send()
        .await?;
    let lists = response.json::<Vec<List>>().await?;
    Ok(lists)
}

pub async fn add_list(list: NewList) -> Result<List, Error> {
    let client = reqwest::Client::new();
    let response = client
        .post(format!("{BASE_URL}/lists"))
        .json(&list)
        .send()
        .await?;
    let list = response.json::<List>().await?;
    Ok(list)
}

pub async fn delete_list(uuid: &str) -> Result<StatusCode, Error> {
    let client = reqwest::Client::new();
    let response = client
        .delete(format!("{BASE_URL}/lists/{uuid}"))
        .send()
        .await?;
    Ok(response.status())
}

pub async fn update_list(uuid: &str, list: UpdateList) -> Result<StatusCode, Error> {
    let client = reqwest::Client::new();
    let response = client
        .put(format!("{BASE_URL}/lists/{uuid}"))
        .json(&list)
        .send()
        .await?;
    Ok(response.status())
}

pub async fn switch_lists(uuid_from: &str, uuid_to: &str) -> Result<StatusCode, Error> {
    let client = reqwest::Client::new();
    let response = client
        .put(format!("{BASE_URL}/lists/switch"))
        .json(&(uuid_from, uuid_to))
        .send()
        .await?;
    Ok(response.status())
}
