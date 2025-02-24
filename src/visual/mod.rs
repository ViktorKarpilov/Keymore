use crate::locator::locator::Locator;
use iced::daemon::Appearance;
use iced::widget::canvas;
use iced::{window, Point, Size};
use iced::{Element, Task};
use iced::{Length, Theme};
use locators_canvas::LocatorCanvas;
use screen_size::get_primary_screen_size;
use std::sync::mpsc::channel;
use windows::Win32::Foundation::POINT;
mod key_queue;
mod locators_canvas;

trait ToPoint {
    fn to_point(win_point: POINT) -> Point {
        Point {
            x: win_point.x as f32,
            y: (-win_point.y) as f32,
        }
    }
}

pub struct TransparentLayout {
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
impl TransparentLayout {
    pub fn create_layout(locators: Vec<Locator>) -> Result<Option<Locator>, iced::Error> {
        let (tx, rx) = channel::<Locator>();

        let (width, height) = get_primary_screen_size().expect("Screen size");
        let size: Size = Size::new(width as f32, height as f32);

        let _ = iced::application(
            "Keymore layout selector",
            TransparentLayout::update,
            TransparentLayout::view,
        )
        .window_size(size)
        .decorations(false)
        .centered()
        .transparent(true)
        .style(TransparentLayout::style)
        .run_with(|| {
            (
                TransparentLayout {
                    locators,
                    chosen_locator: None,
                    sender: tx,
                },
                window::get_latest().and_then(|id| window::gain_focus(id)),
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
        canvas(LocatorCanvas {
            locators: &self.locators,
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
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
