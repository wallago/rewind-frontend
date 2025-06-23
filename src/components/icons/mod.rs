use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct IconProps {
    #[props(default = String::from("24px"))]
    pub size: String,
}

mod contrast;
pub use contrast::Contrast;

mod arrow;
pub use arrow::Bottom as ArrowBottom;

mod more;
pub use more::More;

mod cross;
pub use cross::Cross;

mod settings;
pub use settings::Settings;

mod add;
pub use add::Add;
