use reqwest::Error;

use crate::{
    api::BASE_URL,
    models::{NewTag, Tag},
};

pub async fn get_tags() -> Result<Vec<Tag>, Error> {
    let client = reqwest::Client::new();
    let response = client.get(format!("{BASE_URL}/tags")).send().await?;
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
