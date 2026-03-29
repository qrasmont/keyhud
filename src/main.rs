use iced::{
    Application, Background, Color, Element,
    Length::Fill,
    Program, Subscription, Task, Theme,
    futures::{SinkExt, Stream, StreamExt, channel::mpsc},
    stream,
    window::{Event as WEvent, Id, Level},
};
use rdev::{Event, EventType, Key, grab};

pub fn main() -> iced::Result {
    application().run()
}

fn application() -> Application<impl Program<Message = Message, Theme = Theme>> {
    iced::application(App::new, App::update, App::view)
        .subscription(App::subscription)
        .title("keyhud")
        .transparent(true)
        .decorations(false)
        .theme(App::theme)
        .window_size((500.0, 500.0))
        .level(Level::AlwaysOnTop)
}

#[derive(Default)]
struct App {
    keys: Vec<String>,
}

#[derive(Debug, Clone)]
enum Message {
    KeyPressed(Key),
    KeyReleased(Key),
    WindowOpened(Id),
}

impl App {
    fn new() -> (App, Task<Message>) {
        (App::default(), Task::none())
    }

    fn update(state: &mut App, message: Message) -> Task<Message> {
        match message {
            Message::KeyPressed(key) => state.keys.push(format!("p {key:?}")),
            Message::KeyReleased(key) => state.keys.push(format!("d {key:?}")),
            Message::WindowOpened(id) => {
                return iced::window::enable_mouse_passthrough(id);
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let entries = self
            .keys
            .iter()
            .rev()
            .map(|line| iced::widget::text(line).into())
            .collect::<Vec<_>>();

        let content = iced::widget::scrollable(iced::widget::column(entries).spacing(4));

        iced::widget::container(content)
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

    fn subscription(_state: &App) -> Subscription<Message> {
        let keys = Subscription::run(global_key_work);

        let window_opened = iced::event::listen_with(|event, _, id| match event {
            iced::Event::Window(WEvent::Opened { .. }) => Some(Message::WindowOpened(id)),
            _ => None,
        });

        Subscription::batch([keys, window_opened])
    }
}

fn global_key_work() -> impl Stream<Item = Message> {
    stream::channel(100, async |mut output| {
        let (tx, mut rx) = mpsc::unbounded::<Event>();

        std::thread::spawn(move || {
            grab(move |event: Event| -> Option<Event> {
                let _ = tx.unbounded_send(event.clone());
                Some(event)
            })
        });

        loop {
            if let Some(event) = rx.next().await {
                let msg = match event.event_type {
                    EventType::KeyPress(key) => Some(Message::KeyPressed(key)),
                    EventType::KeyRelease(key) => Some(Message::KeyReleased(key)),
                    _ => None,
                };

                if let Some(msg) = msg {
                    let _ = output.send(msg).await;
                }
            }
        }
    })
}
