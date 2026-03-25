use iced::{Application, Background, Color, Element, Length::Fill, Program, Task, Theme};

pub fn main() -> iced::Result {
    application().run()
}

fn application() -> Application<impl Program<Message = Message, Theme = Theme>> {
    iced::application(App::new, App::update, App::view)
        .title("keyhud")
        .transparent(true)
        .decorations(false)
        .theme(App::theme)
        .window_size((500.0, 500.0))
}

#[derive(Default)]
struct App;

#[derive(Debug, Clone)]
enum Message {}

impl App {
    fn new() -> (App, Task<Message>) {
        (App::default(), Task::none())
    }

    fn update(_state: &mut App, _message: Message) -> Task<Message> {
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        iced::widget::container(iced::widget::Space::new())
            .width(Fill)
            .height(Fill)
            .style(|_theme| iced::widget::container::Style {
                background: Some(Background::Color(Color::TRANSPARENT)),
                ..Default::default()
            })
            .into()
    }

    fn theme(_state: &App) -> Theme {
        Theme::custom(
            "transparent".to_string(),
            iced::theme::Palette {
                background: Color::TRANSPARENT,
                text: Color::WHITE,
                primary: Color::WHITE,
                success: Color::WHITE,
                danger: Color::WHITE,
                warning: Color::WHITE,
            },
        )
    }
}
