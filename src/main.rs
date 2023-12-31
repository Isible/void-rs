use iced::theme::Button;
use iced::widget::button::StyleSheet;
use iced::{executor, Color};
use iced::widget::{button, column, container};
use iced::{Alignment, Application, Command, Element, Length, Settings, Theme};
use logger::Logger;
use translations::Translator;

mod logger;
mod translations;

pub const NAME: &str = "Void";
pub const LOGGER: Logger = Logger::new(NAME);

pub fn main() -> iced::Result {
    VoidApp::run(Settings::default())
}

struct VoidApp<'a> {
    translator: Translator<'a>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    CreateProj,
    CloneProj,
    OpenProj,
}

impl Application for VoidApp<'_> {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                translator: Translator::new(translations::DEFAULT_LANGUAGE),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        self.translator.load("window.title").into()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::CreateProj => todo!(),
            Message::CloneProj => todo!(),
            Message::OpenProj => todo!(),
        }
    }

    fn view(&self) -> Element<Message> {
        let content = column![
            button(self.translator.load("buttons.start.create-proj"))
                .padding([10, 20])
                .on_press(Message::CreateProj),
            button(self.translator.load("buttons.start.clone-proj"))
                .padding([10, 20])
                .on_press(Message::CloneProj),
            button(self.translator.load("buttons.start.open-proj"))
                .padding([10, 20])
                .on_press(Message::OpenProj),
            button("deez")
                .padding([50, 20])
                .on_press(Message::OpenProj),
        ]
        .spacing(10)
        .align_items(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .center_x()
            .center_y()
            .into()
    }
}
/*
#[derive(Default)]
struct ButtonStyle;

impl button::StyleSheet for ButtonStyle {
    type Style = ;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(Color::from_rgb8(0x4c, 0xaf, 0x50))), // Set the background color here
            text_color: iced::Color::WHITE,
            ..button::Appearance::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        self.active(style)
    }
}
*/