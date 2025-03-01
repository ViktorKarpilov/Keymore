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
use std::sync::mpsc::channel;
mod key_queue;
mod locators_canvas;
mod locators_trie_node;

pub struct TransparentLayout {
    chosen_locator: Option<Locator>,
    chosen_key: Option<String>,
    sender: std::sync::mpsc::Sender<Locator>,
    canvas_layout: LocatorCanvas,
}

impl TransparentLayout {
    pub fn new(
        locators_trie: LocatorTrieNode,
        chosen_locator: Option<Locator>,
        chosen_key: Option<String>,
        sender: std::sync::mpsc::Sender<Locator>,
    ) -> TransparentLayout {
        let canvas: LocatorCanvas = LocatorCanvas::new(locators_trie, chosen_key.clone());

        TransparentLayout {
            chosen_key,
            sender,
            chosen_locator,
            canvas_layout: canvas,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    LocatorChosen(Locator),
    Dismiss,
    UpdateChosenKey(smol_str::SmolStr),
    Pass,
}

// Locators in are not the same as locators used - need to fix that shit
impl TransparentLayout {
    pub fn create_layout(locators: Vec<Locator>) -> Result<Option<Locator>, iced::Error> {
        let locators_trie = LocatorTrieNode::new(locators);
        let (tx, rx) = channel::<Locator>();
        let layout: TransparentLayout = TransparentLayout::new(locators_trie, None, None, tx);

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
                layout,
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
            Message::LocatorChosen(locator) => {
                let _ = self.sender.send(locator);
                window::get_latest().and_then(window::close)
            }
            Message::UpdateChosenKey(new_key) => {
                let new_key = {
                    match potential_targets.contains(&new_key.as_str()) {
                        true => Some({
                            if let Some(chosen_key) = &self.chosen_key {
                                format!("{}{}", chosen_key, new_key.as_str())
                            } else {
                                format!("{}", new_key.as_str())
                            }
                        }),
                        false => None,
                    }
                };
                
                println!("New key: {:?}", new_key);
                self.canvas_layout.update(new_key);
                Task::none()
            }
            Message::Dismiss => window::get_latest().and_then(window::close),
            Message::Pass => Task::none(),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        canvas(self.canvas_layout.clone())
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
