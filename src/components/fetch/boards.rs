use dioxus::prelude::*;
use futures_timer::Delay;
use reqwest::Error;
use serde::Serialize;

use super::API;
use crate::views::Board;

pub fn get_boards(refetch_signal: Signal<u32>) -> Resource<Option<Vec<Board>>> {
    use_resource(move || {
        let _ = refetch_signal();
        async move {
            let mut retries = 0;
            loop {
                match reqwest::get(format!("{API}/boards")).await {
                    Ok(res) => match res.json::<Vec<Board>>().await {
                        Ok(boards) => break Some(boards),
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
pub struct NewBoard {
    pub name: String,
    pub description: Option<String>,
    pub position: i32,
}

pub async fn add_board(board: NewBoard) -> Result<Board, Error> {
    let client = reqwest::Client::new();
    let response = client
        .post(format!("{API}/boards"))
        .json(&board)
        .send()
        .await?;
    let board = response.json::<Board>().await?;
    tracing::info!("New board has been added: {}", board.uuid);
    Ok(board)
}

pub async fn delete_board(uuid: &str) -> Result<bool, Error> {
    let client = reqwest::Client::new();
    let response = client.delete(format!("{API}/boards/{uuid}")).send().await?;
    if response.status().is_success() {
        Ok(true)
    } else {
        Ok(false)
    }
}

#[derive(Serialize, Clone)]
pub struct UpdateBoard {
    pub name: Option<String>,
    pub description: Option<String>,
    pub position: Option<i32>,
}

pub async fn update_board(uuid: &str, board: UpdateBoard) -> Result<bool, Error> {
    let client = reqwest::Client::new();
    let response = client
        .put(format!("{API}/boards/{uuid}"))
        .json(&board)
        .send()
        .await?;
    if response.status().is_success() {
        Ok(true)
    } else {
        Ok(false)
    }
}

pub async fn switch_boards(uuid_from: &str, uuid_to: &str) -> Result<bool, Error> {
    let client = reqwest::Client::new();
    let response = client
        .put(format!("{API}/boards/switch"))
        .json(&(uuid_from, uuid_to))
        .send()
        .await?;
    if response.status().is_success() {
        Ok(true)
    } else {
        Ok(false)
    }
}
