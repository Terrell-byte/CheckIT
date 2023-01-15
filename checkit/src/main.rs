mod view;
use iced::{Application, Settings, executor};
use view::AppInit;
pub fn main() -> iced::Result {
    AppInit::run(Settings::default())
}
