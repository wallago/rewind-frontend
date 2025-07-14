use reqwest::{Error, StatusCode};

use crate::{
    api::BASE_URL,
    models::{Board, NewBoard},
};

pub async fn get_boards() -> Result<Vec<Board>, Error> {
    let client = reqwest::Client::new();
    let response = client.get(format!("{BASE_URL}/boards")).send().await?;
    let boards = response.json::<Vec<Board>>().await?;
    Ok(boards)
}

pub async fn add_board(board: NewBoard) -> Result<Board, Error> {
    let client = reqwest::Client::new();
    let response = client
        .post(format!("{BASE_URL}/boards"))
        .json(&board)
        .send()
        .await?;
    let board = response.json::<Board>().await?;
    Ok(board)
}

pub async fn delete_board(board_uuid: String) -> Result<StatusCode, Error> {
    let client = reqwest::Client::new();
    let response = client
        .delete(format!("{BASE_URL}/boards/{board_uuid}"))
        .send()
        .await?;
    Ok(response.status())
}
