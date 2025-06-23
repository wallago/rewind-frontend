use dioxus::prelude::*;
use serde::Deserialize;

use crate::components::{
    board::{
        Board,
        row::{BoardRow, BoardRowSkeleton, BoardRowTitle},
    },
    icons::arrow,
};

static API: &str = "http://0.0.0.0:8081/api";

pub fn get_boards() -> Result<Resource<Option<Vec<Board>>>, RenderError> {
    let boards = use_resource(move || async move {
        let response = reqwest::get(format!("{API}/boards",)).await.ok()?;
        response.json::<Vec<Board>>().await.ok()
    });
    Ok(boards)
}
