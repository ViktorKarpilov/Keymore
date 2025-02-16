use iced::daemon::Appearance;
use iced::widget::button;
use iced::Theme;
use iced::{window, Point};
use iced::{Element, Task};
use windows::Win32::Foundation::POINT;

use crate::locator::locator::Locator;

struct IcedPoint;

trait ToPoint {
    fn to_point(win_point: POINT) -> Point {
        Point {
            x: win_point.x as f32,
            y: (-win_point.y) as f32,
        }
    }
}

#[derive(Default)]
pub struct TransparantLayout {
    pub locators: Vec<Locator>,
    pub chosen_locator: Option<Locator>,
}

#[derive(Debug, Clone)]
enum Message {
    LocatorChoosen(Locator),
    Dismiss,
}

// Locators in are not the same as locators used - need to fix that shit
impl TransparantLayout {
    pub fn create_layout(
        &mut self,
        locators: Vec<Locator>,
    ) -> Result<Option<Locator>, iced::Error> {
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
                },
                Task::none(),
            )
        });

        return Ok(match self.chosen_locator.clone() {
            Some(value) => Some(value),
            None => None,
        });
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::LocatorChoosen(locator) => {
                self.chosen_locator = Some(locator);
                Task::none()
            }
            Message::Dismiss => window::get_latest().and_then(window::close),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let test_button: Element<'_, Message> =
            button("Close me!").on_press(Message::Dismiss).into();

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

pub trait Render {
    fn render(&self) {
        todo!()
    }
}
