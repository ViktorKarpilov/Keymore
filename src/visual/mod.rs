use crate::key_qeue_14;
use crate::locator::locator::Locator;
use iced::daemon::Appearance;
use iced::widget::canvas;
use iced::{keyboard, window, Size, Subscription};
use iced::{Element, Task};
use iced::{Length, Theme};
use locators_canvas::LocatorCanvas;
use locators_trie_node::LocatorTrieNode;
use screen_size::get_primary_screen_size;
use std::str::FromStr;
use std::sync::mpsc::channel;
mod key_queue;
mod locators_canvas;
mod locators_trie_node;

pub struct TransparentLayout {
    locators_trie: LocatorTrieNode,
    chosen_locator: Option<Locator>,
    chosen_key: String,
    sender: std::sync::mpsc::Sender<Locator>,
}

#[derive(Debug, Clone)]
pub enum Message {
    LocatorChoosen(Locator),
    Dismiss,
    UpdateChosenKey(smol_str::SmolStr),
    Pass,
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
        .subscription(TransparentLayout::subscription)
        .run_with(|| {
            (
                TransparentLayout {
                    locators_trie,
                    chosen_locator: None,
                    chosen_key: String::new(),
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
        let potential_targets = key_qeue_14!();

        match message {
            Message::LocatorChoosen(locator) => {
                let _ = self.sender.send(locator);
                window::get_latest().and_then(window::close)
            }
            Message::UpdateChosenKey(new_key) => {
                self.chosen_key = {
                    match potential_targets.contains(&new_key.as_str()) {
                        true => format!("{}{}", self.chosen_key, new_key.as_str()),
                        false => String::new(),
                    }
                };

                Task::none()
            }
            Message::Dismiss => window::get_latest().and_then(window::close),
            Message::Pass => Task::none(),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let location_key = match self.chosen_key.as_ref() {
            "" => None,
            non_trivial_string => Some(non_trivial_string.to_string()),
        };

        canvas(LocatorCanvas {
            locators_trie: &self.locators_trie,
            location_key,
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        keyboard::on_key_press(|key, _modifiers| match key {
            keyboard::Key::Character(symbol) => Some(Message::UpdateChosenKey(symbol)),
            _ => None,
        })
    }

    fn style(&self, theme: &Theme) -> Appearance {
        Appearance {
            background_color: iced::Color::TRANSPARENT,
            text_color: theme.palette().text,
        }
    }
}
