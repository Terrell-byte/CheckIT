use iced::executor;
use iced::{Application, Command, Element, Settings, Theme};

pub(crate) struct AppInit;

impl Application for AppInit {
    type Executor = executor::Default;
    type Flags = ();
    type Message = ();
    type Theme = Theme;

    fn new(_flags: ()) -> (AppInit, Command<Self::Message>) {
        (AppInit, Command::none())
    }

    fn title(&self) -> String {
        String::from("CheckIt")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        "Hello, world!".into()
    }
}