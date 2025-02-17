use iced::daemon::Appearance;
use iced::widget::{button, Button};
use iced::Theme;
use iced::{window, Point};
use iced::{Element, Task};
use std::sync::mpsc::channel;
use windows::Win32::Foundation::POINT;

use crate::locator::locator::Locator;

trait ToPoint {
    fn to_point(win_point: POINT) -> Point {
        Point {
            x: win_point.x as f32,
            y: (-win_point.y) as f32,
        }
    }
}

pub struct TransparantLayout {
    locators: Vec<Locator>,
    chosen_locator: Option<Locator>,
    sender: std::sync::mpsc::Sender<Locator>,
}

#[derive(Debug, Clone)]
pub enum Message {
    LocatorChoosen(Locator),
    Dismiss,
}

// Locators in are not the same as locators used - need to fix that shit
impl TransparantLayout {
    pub fn create_layout(locators: Vec<Locator>) -> Result<Option<Locator>, iced::Error> {
        let (tx, rx) = channel::<Locator>();

        let _ = iced::application(
            "Keymore layout selector",
            TransparantLayout::update,
            TransparantLayout::view,
        )
        .transparent(true)
        .decorations(false)
        .style(TransparantLayout::style)
        .run_with(|| {
            (
                TransparantLayout {
                    locators,
                    chosen_locator: None,
                    sender: tx,
                },
                Task::none(),
            )
        });

        Ok(match rx.recv() {
            Ok(value) => Some(value),
            Err(_) => None,
        })
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::LocatorChoosen(locator) => {
                let _ = self.sender.send(locator);
                window::get_latest().and_then(window::close)
            }
            Message::Dismiss => window::get_latest().and_then(window::close),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let test_button: Element<'_, Message> = button("Close me!")
            .on_press(Message::LocatorChoosen(Locator {
                point: POINT { x: 2, y: 4 },
            }))
            .into();

        test_button
    }

    // fn subscription(&self) -> Subscription<Message> {
    //     keyboard::on_key_press(|key, _modifiers| match key {
    //         keyboard::Key::Named(
    //             keyboard::key::Named::ArrowUp | keyboard::key::Named::ArrowLeft,
    //         ) => Some(Message::PreviousTheme),
    //         keyboard::Key::Named(
    //             keyboard::key::Named::ArrowDown | keyboard::key::Named::ArrowRight,
    //         ) => Some(Message::NextTheme),
    //         _ => None,
    //     })
    // }

    fn style(&self, theme: &Theme) -> Appearance {
        Appearance {
            background_color: iced::Color::TRANSPARENT,
            text_color: theme.palette().text,
        }
    }
}

pub trait RenderButton {
    fn render(&self) -> Button<'_, Message> {}
}
