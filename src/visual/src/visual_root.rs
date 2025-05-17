use iced::{keyboard, window, Element, Subscription, Task, Theme};
use iced::application::Appearance;

#[derive(Debug, Clone)]
pub  enum RootMessage {
    Exit,
}

#[derive(Default)]
pub  struct VisualRoot {}

impl VisualRoot {
    pub fn update(&mut self, message: RootMessage) -> Task<RootMessage> {
        match message {
            RootMessage::Exit => {
                window::get_latest().and_then(window::close)
            }
        }

    }

    pub  fn view(&self) -> Element<'_, RootMessage> {
        iced::widget::text("Some cool text").into()
    }

    pub  fn subscription(&self) -> Subscription<RootMessage> {
        keyboard::on_key_press(|key, _modifiers| match key {
            keyboard::Key::Named(keyboard::key::Named::Escape) => Some(RootMessage::Exit),
            // keyboard::Key::Character(symbol) => Some(Message::UpdateChosenKey(symbol)),
            _ => None,
        })
    }

    pub  fn style(&self, theme: &Theme) -> Appearance {
        Appearance {
            // background_color: iced::Color::TRANSPARENT,
            background_color: iced::Color::from_rgb(0.7, 0.7, 0.7),
            text_color: theme.palette().text,
        }
    }
}
