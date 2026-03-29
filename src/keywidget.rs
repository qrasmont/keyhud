use iced::{Background, Color, Element};

use rdev::Key;

use crate::app::Message;

pub const BASE_KEY_SIZE: f32 = 40.0;

pub fn keyboard_layout() -> Vec<Vec<Option<(Key, &'static str, f32)>>> {
    vec![
        vec![
            Some((Key::Escape, "Esc", 1.0)),
            None,
            Some((Key::F1, "F1", 1.0)),
            Some((Key::F2, "F2", 1.0)),
            Some((Key::F3, "F3", 1.0)),
            Some((Key::F4, "F4", 1.0)),
            Some((Key::F5, "F5", 1.0)),
            Some((Key::F6, "F6", 1.0)),
            Some((Key::F7, "F7", 1.0)),
            Some((Key::F8, "F8", 1.0)),
            Some((Key::F9, "F9", 1.0)),
            Some((Key::F10, "F10", 1.0)),
            Some((Key::F11, "F11", 1.0)),
            Some((Key::F12, "F12", 1.0)),
            None,
        ],
        vec![
            Some((Key::Num1, "1", 1.0)),
            Some((Key::Num2, "2", 1.0)),
            Some((Key::Num3, "3", 1.0)),
            Some((Key::Num4, "4", 1.0)),
            Some((Key::Num5, "5", 1.0)),
            Some((Key::Num6, "6", 1.0)),
            Some((Key::Num7, "7", 1.0)),
            Some((Key::Num8, "8", 1.0)),
            Some((Key::Num9, "9", 1.0)),
            Some((Key::Num0, "0", 1.0)),
            Some((Key::Minus, ")", 1.0)),
            Some((Key::Equal, "-", 1.0)),
            Some((Key::Backspace, "⌫", 2.0)),
        ],
        vec![
            Some((Key::Tab, "Tab", 1.5)),
            Some((Key::KeyQ, "A", 1.0)),
            Some((Key::KeyW, "Z", 1.0)),
            Some((Key::KeyE, "E", 1.0)),
            Some((Key::KeyR, "R", 1.0)),
            Some((Key::KeyT, "T", 1.0)),
            Some((Key::KeyY, "Y", 1.0)),
            Some((Key::KeyU, "U", 1.0)),
            Some((Key::KeyI, "I", 1.0)),
            Some((Key::KeyO, "O", 1.0)),
            Some((Key::KeyP, "P", 1.0)),
            Some((Key::LeftBracket, "^", 1.0)),
            Some((Key::RightBracket, "$", 1.0)),
            Some((Key::Return, "⏎", 1.0)),
        ],
        vec![
            Some((Key::CapsLock, "", 2.0)),
            Some((Key::KeyA, "Q", 1.0)),
            Some((Key::KeyS, "S", 1.0)),
            Some((Key::KeyD, "D", 1.0)),
            Some((Key::KeyF, "F", 1.0)),
            Some((Key::KeyG, "G", 1.0)),
            Some((Key::KeyH, "H", 1.0)),
            Some((Key::KeyJ, "J", 1.0)),
            Some((Key::KeyK, "K", 1.0)),
            Some((Key::KeyL, "L", 1.0)),
            Some((Key::SemiColon, "M", 1.0)),
            Some((Key::Quote, "ù", 1.0)),
            Some((Key::BackSlash, "`", 1.0)),
        ],
        vec![
            Some((Key::ShiftLeft, "", 1.2)),
            Some((Key::BackQuote, "<", 1.0)),
            Some((Key::KeyZ, "W", 1.0)),
            Some((Key::KeyX, "X", 1.0)),
            Some((Key::KeyC, "C", 1.0)),
            Some((Key::KeyV, "V", 1.0)),
            Some((Key::KeyB, "B", 1.0)),
            Some((Key::KeyN, "N", 1.0)),
            Some((Key::KeyM, ",", 1.0)),
            Some((Key::Comma, ";", 1.0)),
            Some((Key::Dot, ":", 1.0)),
            Some((Key::Slash, "=", 1.0)),
            Some((Key::ShiftRight, "", 2.5)),
        ],
        vec![
            None,
            None,
            Some((Key::ControlLeft, "Ctrl", 1.0)),
            Some((Key::Alt, "Opt", 1.0)),
            Some((Key::MetaLeft, "Cmd", 1.0)),
            Some((Key::Space, "", 5.0)),
            Some((Key::MetaRight, "Cmd", 1.0)),
            Some((Key::AltGr, "Opt", 1.0)),
        ],
    ]
}

pub fn key_widget<'a>(label: &'a str, width: f32, pressed: bool) -> Element<'a, Message> {
    let bg_color = if pressed {
        Color::from_rgba(1.0, 1.0, 1.0, 0.85)
    } else {
        Color::from_rgba(1.0, 1.0, 1.0, 0.05)
    };

    let text_color = if pressed { Color::BLACK } else { Color::WHITE };

    iced::widget::container(iced::widget::text(label).color(text_color).size(11))
        .width(BASE_KEY_SIZE * width)
        .height(BASE_KEY_SIZE)
        .align_x(iced::alignment::Horizontal::Center)
        .align_y(iced::alignment::Vertical::Center)
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(bg_color)),
            border: iced::Border {
                color: Color::from_rgba(1.0, 1.0, 1.0, 0.6),
                width: 1.0,
                radius: 4.0.into(),
            },
            ..Default::default()
        })
        .into()
}
