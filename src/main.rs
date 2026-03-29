use iced::{
    Application, Color, Element, Program, Subscription, Task, Theme,
    futures::{SinkExt, Stream, StreamExt, channel::mpsc},
    stream,
    window::{Event as WEvent, Level, Position},
};
use rdev::{Event, EventType, grab};

mod keywidget;
use keywidget::{BASE_KEY_SIZE, key_widget, keyboard_layout};

mod app;
use app::{App, Message};

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
        .window_size((680.0, 320.0))
        .level(Level::AlwaysOnTop)
        .position(Position::Centered)
}

impl App {
    fn new() -> (App, Task<Message>) {
        (App::default(), Task::none())
    }

    fn update(state: &mut App, message: Message) -> Task<Message> {
        match message {
            Message::KeyPressed(key) => {
                state.pressed.insert(key);
            }
            Message::KeyReleased(key) => {
                state.pressed.remove(&key);
            }
            Message::WindowOpened(id) => {
                return iced::window::enable_mouse_passthrough(id);
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let rows = keyboard_layout().into_iter().map(|row| {
            let keys = row.into_iter().map(|slot| match slot {
                Some((key, label, w)) => key_widget(label, w, self.pressed.contains(&key)),
                None => iced::widget::Space::new().width(BASE_KEY_SIZE * 0.5).into(),
            });
            iced::widget::row(keys).spacing(4).into()
        });

        iced::widget::column(rows).spacing(4).padding(12).into()
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
