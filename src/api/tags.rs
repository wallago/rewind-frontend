use reqwest::Error;

use crate::{api::BASE_URL, models::Tag};

pub async fn get_tags() -> Result<Vec<Tag>, Error> {
    let client = reqwest::Client::new();
    let response = client.get(format!("{BASE_URL}/tags")).send().await?;
    let tags = response.json::<Vec<Tag>>().await?;
    Ok(tags)
}
