// pub const BASE_URL: &str = "http://0.0.0.0:8081/api";
pub const BASE_URL: &str = "https://rewind.henrotte.work/api";

mod boards;
pub use boards::{add_board, delete_board, get_boards, switch_boards, update_board};

mod lists;
pub use lists::{add_list, get_lists_by_board_uuid, switch_lists, update_list};

mod tasks;
pub use tasks::{add_task, get_tasks_by_list_uuid, switch_tasks, update_task};

mod tags;
pub use tags::{add_tag, get_tags};
