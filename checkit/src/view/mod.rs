use iced::{Application, executor};
use iced::widget::{button, column, text, Column};

pub(crate) struct AppInit {
}

impl Application for AppInit {
    type Executor = executor::Default;
    type Message = ();
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (AppInit {}, iced::Command::none())
    }

    fn title(&self) -> String {
        String::from("CheckIT")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        iced::Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        todo!()
    }

    type Theme = iced::Theme;
}