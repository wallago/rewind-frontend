use dioxus::prelude::*;

#[derive(Debug, Clone)]
pub struct ThemeContext(pub Signal<bool>);
