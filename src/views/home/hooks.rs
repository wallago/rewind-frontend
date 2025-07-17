use dioxus::prelude::*;

use crate::{api::update_board, context::BoardsContext, models::UpdateBoard};

// fn use_board_drag_switch(
//     cx: Scope<BoardCardProps>,
//     dragging_from: &Signal<Option<String>>,
//     dragging_to: &UseState<Option<String>>,
//     ctx_boards: &BoardsContext,
// ) {
//     let trigger_switch = use_signal(cx, || false);
//     let in_progress_switch = use_signal(cx, || false);

//     use_effect(cx, (trigger_switch.get(),), move |(trigger)| {
//         if *trigger {
//             let from = dragging_from.get();
//             let to = dragging_to.get();
//             if let (Some(uuid_from), Some(uuid_to)) = (from.as_ref(), to.as_ref()) {
//                 to_owned!((
//                     ctx_boards,
//                     in_progress_switch,
//                     trigger_switch,
//                     dragging_from,
//                     dragging_to
//                 ));
//                 cx.spawn(async move {
//                     in_progress_switch.set(true);
//                     let res = switch_boards(uuid_from, uuid_to).await;
//                     if res.is_ok() {
//                         ctx_boards.refresh.set(());
//                     }
//                     in_progress_switch.set(false);
//                     dragging_from.set(None);
//                     dragging_to.set(None);
//                     trigger_switch.set(false);
//                 });
//             }
//         }
//         None
//     });

//     // Setup event handlers for drag-drop and setting dragging_to / trigger_switch
// }
