use crate::{key_qeue_14, visual};
use crate::windows::locator::locator::Locator;
use iced::daemon::Appearance;
use iced::widget::canvas;
use iced::{keyboard, window, Size, Subscription};
use iced::{Element, Task};
use iced::{Length, Theme};
use crate::visual::layout::locators::locators_canvas::LocatorCanvas;
use crate::visual::layout::locators::locators_trie_node::LocatorTrieNode;
use screen_size::get_primary_screen_size;
use std::sync::mpsc::channel;
use log::trace;
use crate::visual::layout::vignette_canvas::VignetteCanvas;

pub struct TransparentLayout {
    sender: std::sync::mpsc::Sender<Locator>,
    pub locators_canvas: LocatorCanvas,
}

impl TransparentLayout {
    pub fn new(
        locators_trie: LocatorTrieNode,
        sender: std::sync::mpsc::Sender<Locator>,
    ) -> TransparentLayout {
        let canvas: LocatorCanvas = LocatorCanvas::new(locators_trie, None);

        TransparentLayout {
            sender,
            locators_canvas: canvas,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    LocatorChosen(Locator),
    UpdateChosenKey(smol_str::SmolStr),
    Exit,
}

// Locators in are not the same as locators used - need to fix that shit
impl TransparentLayout {
    pub fn create_layout(locators: Vec<Locator>) -> Result<Option<Locator>, iced::Error> {
        let locators_trie = LocatorTrieNode::new(locators);
        let (tx, rx) = channel::<Locator>();

        let layout: TransparentLayout = TransparentLayout::new(locators_trie, tx);

        let (width, height) = get_primary_screen_size().expect("Screen size");
        let size: Size = Size::new(width as f32, height as f32);

        let _ =
            iced::application(
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

    pub fn update(&mut self, message: Message) -> Task<Message> {
        let potential_targets = key_qeue_14!();

        match message {
            Message::Exit => {
                window::get_latest().and_then(window::close)
            }
            Message::LocatorChosen(locator) => {
                let _ = self.sender.send(locator);
                Task::done(Message::Exit)
            }
            Message::UpdateChosenKey(new_key) => {
                let new_key = {
                    match potential_targets.contains(&new_key.as_str()) {
                        true => Some({
                            if let Some(chosen_key) = &self.locators_canvas.location_key {
                                format!("{}{}", chosen_key, new_key.as_str())
                            } else {
                                format!("{}", new_key.as_str())
                            }
                        }),
                        false => None,
                    }
                };

                self.locators_canvas.update(new_key.clone());

                if self.locators_canvas.locations_paths.is_none() {
                    self.locators_canvas.update(None);
                }

                if let Some(points) = &self.locators_canvas.locations_paths {
                    if points.len() == 1 {
                        return Task::done(Message::LocatorChosen(
                            points[0].0.node.clone().unwrap(),
                        ));
                    }
                }

                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        trace!("Draw a rectangle");
        canvas(VignetteCanvas { border_width: 20.0, opacity: 0.2 })
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
        
        // canvas(self.locators_canvas.clone())
        //     .width(Length::Fill)
        //     .height(Length::Fill)
        //     .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        keyboard::on_key_press(|key, _modifiers| match key {
            keyboard::Key::Named(keyboard::key::Named::Escape) => Some(Message::Exit),
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
