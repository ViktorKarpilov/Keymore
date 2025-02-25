use crate::locator::locator::Locator;
use iced::daemon::Appearance;
use iced::widget::canvas;
use iced::{window, Size};
use iced::{Element, Task};
use iced::{Length, Theme};
use locators_canvas::LocatorCanvas;
use locators_trie_node::LocatorTrieNode;
use screen_size::get_primary_screen_size;
use std::sync::mpsc::channel;
mod key_queue;
mod locators_canvas;
mod locators_trie_node;

pub struct TransparentLayout {
    locators_trie: LocatorTrieNode,
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
        let locators_trie = LocatorTrieNode::new(locators);
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
                    locators_trie,
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
            locators_trie: &self.locators_trie,
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
