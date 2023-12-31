use iced::executor;
use iced::widget::{button, column, container};
use iced::window;
use iced::{Alignment, Application, Command, Element, Length, Settings, Theme};
use logger::Logger;
use translations::Translator;

mod translations;
mod logger;

pub const NAME: &str = "Void";
pub const LOGGER: Logger = Logger::new(NAME);

pub fn main() -> iced::Result {
    VoidApp::run(Settings::default())
}

struct VoidApp<'a> {
    show_confirm: bool,
    translator: Translator<'a>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Confirm,
    Exit,
    ChangeLang,
    CreateProj,
}

impl Application for VoidApp<'_> {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                show_confirm: false,
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
            Message::Confirm => window::close(),
            Message::Exit => {
                self.show_confirm = true;
                Command::none()
            }
            Message::ChangeLang => {
                self.translator.change_language(match self.translator.lang() {
                    "de-de" => "en-us",
                    "en-us" => "de-de",
                    _ => panic!()
                });
                Command::none()
            },
            Message::CreateProj => {
                println!("Nuts");
                Command::none()
            },
        }
    }

    fn view(&self) -> Element<Message> {
        let content = if self.show_confirm {
            column![
                "Are you sure you want to exit?",
                button(self.translator.load("buttons.start.clone-proj"))
                    .padding([10, 20])
                    .on_press(Message::Confirm),
            ]
        } else {
            column![
                "Click the button to exit",
                button(self.translator.load("buttons.start.create-proj"))
                    .padding([10, 20])
                    .on_press(Message::Exit),
                button(self.translator.load("buttons.start.change-lang"))
                    .padding([10, 20])
                    .on_press(Message::ChangeLang),
            ]
        }
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
