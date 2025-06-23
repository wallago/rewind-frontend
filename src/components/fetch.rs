use dioxus::prelude::*;
use futures_timer::Delay;
use serde::Serialize;

use crate::components::board::Board;

static API: &str = "http://0.0.0.0:8081/api";

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
}

pub async fn add_board(board: NewBoard) -> Option<Board> {
    let client = reqwest::Client::new();
    let response = client
        .post(format!("{API}/boards"))
        .json(&board)
        .send()
        .await
        .ok()?;
    response.json::<Board>().await.ok()
}
