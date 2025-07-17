use reqwest::Error;

use crate::{
    api::BASE_URL,
    models::{NewTag, Tag},
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
