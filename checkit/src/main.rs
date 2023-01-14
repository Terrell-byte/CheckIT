mod model;
mod view;
use iced::{Application, Settings, executor};
use view::AppInit;
fn main() {
    let settings = Settings::default();
    AppInit::run(settings);
}