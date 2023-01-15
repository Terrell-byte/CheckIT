mod view;
mod model;
use iced::{Settings, Application};
use view::AppInit;
use model::todo;
pub fn main() -> iced::Result {
    AppInit::run(Settings {
        window: iced::window::Settings {
            size: (800, 600),
            resizable: false,
            ..Default::default()
        },
        ..Default::default()
    })
}
