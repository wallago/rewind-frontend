use reqwest::Error;

use crate::{
    api::BASE_URL,
    models::{List, NewList},
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
