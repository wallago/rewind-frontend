use dioxus::prelude::*;

#[derive(serde::Deserialize, Debug, Clone)]
struct Board {
    uuid: String,
    name: String,
    description: Option<String>,
}

#[component]
pub fn Home() -> Element {
    let api = "http://0.0.0.0:8081/api";
    let boards = use_resource(move || async move {
        let response = reqwest::get(format!("{api}/boards",)).await.ok()?;
        response.json::<Vec<Board>>().await.ok()
    });

    rsx! {
        div {
            class: "px-4 py-3 mx-8 my-4 border",
            p {
                class: "text-xl font-bold",
                "Welcom in your task manager!"
            }
            p { "Here's a list of your tasks for this month." }
            match boards.read().as_ref() {
                Some(Some(board_list)) => rsx!(
                    ul {
                        class: "space-y-2",
                        {board_list.iter().map(|board|  {
                            rsx!(
                                li {
                                    key: board.uuid,
                                    class: "p-2 bg-gray-100 rounded",
                                    div {
                                        class: "grid grid-cols-3 gap-4",
                                            p { {board.name.clone()} },
                                            p { {board.uuid.clone()} },
                                            p { {board.description.clone()} },
                                        }
                                    }
                                )
                            })}
                    }
                    div {
                        class: "bg-blue p-4 rounded-lg",
                        "If you can see this styled, Tailwind is working!"
                    }
                ),
                Some(None) => rsx!(p { "Failed to load tasks." }),
                None => rsx!(p { "Loading..." }),
            }
        }
    }
}
