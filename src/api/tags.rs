use reqwest::{Error, StatusCode};

use crate::{
    api::BASE_URL,
    models::{NewTag, Tag, UpdateTag},
};

pub async fn get_tags_by_board_uuid(uuid: &str) -> Result<Vec<Tag>, Error> {
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{BASE_URL}/boards/{uuid}/tags"))
        .send()
        .await?;
    let tags = response.json::<Vec<Tag>>().await?;
    Ok(tags)
}

pub async fn get_tags_by_task_uuid(uuid: &str) -> Result<Vec<Tag>, Error> {
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{BASE_URL}/tasks/{uuid}/tags"))
        .send()
        .await?;
    let tags = response.json::<Vec<Tag>>().await?;
    Ok(tags)
}

pub async fn add_tag(tag: NewTag) -> Result<Tag, Error> {
    let client = reqwest::Client::new();
    let response = client
        .post(format!("{BASE_URL}/tags"))
        .json(&tag)
        .send()
        .await?;
    let tag = response.json::<Tag>().await?;
    Ok(tag)
}

pub async fn delete_tag(uuid: &str) -> Result<StatusCode, Error> {
    let client = reqwest::Client::new();
    let response = client
        .delete(format!("{BASE_URL}/tags/{uuid}"))
        .send()
        .await?;
    Ok(response.status())
}

pub async fn update_tag(uuid: &str, tag: UpdateTag) -> Result<StatusCode, Error> {
    let client = reqwest::Client::new();
    let response = client
        .put(format!("{BASE_URL}/tags/{uuid}"))
        .json(&tag)
        .send()
        .await?;
    Ok(response.status())
}
