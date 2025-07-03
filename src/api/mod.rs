pub const BASE_URL: &str = "http://0.0.0.0:8081/api";

mod boards;
pub use boards::{add_board, get_boards};

mod lists;
pub use lists::{add_list, get_lists_by_board_uuid};

mod tasks;
pub use tasks::{add_task, get_tasks_by_list_uuid};

mod tags;
pub use tags::get_tags;
