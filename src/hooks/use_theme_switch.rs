use dioxus::prelude::*;

use crate::{context::ThemeContext, helpers};

pub fn use_theme_switch() {
    let ctx_theme = use_context::<ThemeContext>();

    use_effect(move || {
        let is_dark = ctx_theme.0();
        helpers::save_dark_mode_preference(is_dark);
        let dom_token_list = helpers::get_dom_token_list();
        if let Some(dom_token_list) = dom_token_list {
            if is_dark {
                let _ = dom_token_list.add_1("dark");
            } else {
                let _ = dom_token_list.remove_1("dark");
            }
        }
    });
}
