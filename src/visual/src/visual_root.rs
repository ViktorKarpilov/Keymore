use crate::locators_canvas::locators_trie_node::LocatorTrieNode;
use crate::locators_canvas::LocatorsCanvas;
use crate::vignette_canvas::VignetteCanvas;
use iced::application::Appearance;
use iced::{keyboard, widget, window, Element, Length, Subscription, Task, Theme};
use windows_operations::locator::actions::locator_finder::get_root_locators;

#[derive(Debug, Clone)]
pub  enum RootMessage {
    Exit,
    LocatorsCanvas,
    Vignette,
}

pub enum AvailableVisible{
    LocatorsCanvas(LocatorsCanvas),
    Vignette(VignetteCanvas),
}

impl Default for AvailableVisible{
    fn default()->AvailableVisible{
        AvailableVisible::Vignette(VignetteCanvas { border_width: 20.0, opacity: 0.2 })
    }
}

#[derive(Default)]
pub  struct VisualRoot
{
    pub initiatedVisual: AvailableVisible
}

impl VisualRoot {
    pub fn update(&mut self, message: RootMessage) -> Task<RootMessage> {
        match message {
            RootMessage::Exit => {
                if matches!(self.initiatedVisual, AvailableVisible::Vignette(_)) {
                    return window::get_latest().and_then(window::close)
                }
               
                Task::done(RootMessage::LocatorsCanvas)
            }
            RootMessage::LocatorsCanvas => {
                println!("Start locators search...");
                let locators = get_root_locators().expect("Can't get root locators");
                println!("Finish locators search...");
                let locators_trie = LocatorTrieNode::new(locators);
                self.initiatedVisual = AvailableVisible::LocatorsCanvas(LocatorsCanvas::new(locators_trie ,None));
                Task::none()
            }
            RootMessage::Vignette => {
                self.initiatedVisual = AvailableVisible::Vignette(VignetteCanvas { border_width: 20.0, opacity: 0.2 });
                Task::none()
            }
        }

    }

    pub  fn view(&self) -> Element<'_, RootMessage> {
        match &self.initiatedVisual {
            AvailableVisible::LocatorsCanvas(locators) => {
                widget::canvas(locators)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .into()
            },
            AvailableVisible::Vignette(vignette) => {
                widget::canvas(vignette)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .into()
            }
        }
    }

    pub  fn subscription(&self) -> Subscription<RootMessage> {
        keyboard::on_key_press(|key, _modifiers| match key {
            keyboard::Key::Named(keyboard::key::Named::Escape) => Some(RootMessage::Exit),
            keyboard::Key::Named(keyboard::key::Named::Space) => Some(RootMessage::LocatorsCanvas),
            // keyboard::Key::Character(symbol) => Some(Message::UpdateChosenKey(symbol)),
            _ => None,
        })
    }

    pub  fn style(&self, theme: &Theme) -> Appearance {
        Appearance {
            background_color: iced::Color::TRANSPARENT,
            // background_color: iced::Color::from_rgb(0.7, 0.7, 0.7),
            text_color: theme.palette().text,
        }
    }
}
