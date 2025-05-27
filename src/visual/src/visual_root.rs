use iced::{keyboard, widget, window, Element, Length, Rectangle, Renderer, Subscription, Task, Theme};
use iced::application::Appearance;
use windows_operations::locator::actions::locator_finder::get_root_locators;
use crate::locators_canvas::locators_trie_node::LocatorTrieNode;
use crate::locators_canvas::LocatorsCanvas;
use crate::vignette_canvas::VignetteCanvas;

#[derive(Debug, Clone)]
pub  enum RootMessage {
    Exit,
    LocatorsCanvas,
    Vignette,
}

enum AvailableVisible{
    LocatorsCanvas(LocatorsCanvas),
    Vignette(VignetteCanvas),
}

pub  trait RootVisible {
    fn view(&self) -> Element<RootMessage>;
}

#[derive(Default)]
pub  struct VisualRoot
{
    pub  initiatedVisual: Option<AvailableVisible>
}

// enum AnyCanvas {
//     Locators(LocatorsCanvas),
//     Vignette(VignetteCanvas),
// }
// 
// impl canvas::Program<RootMessage> for AnyCanvas {
//     type State = ();
// 
//     fn draw(&self, state: &Self::State, renderer: &Renderer, theme: &Theme, bounds: Rectangle, cursor: Cursor) -> Vec<canvas::Geometry> {
//         match self {
//             AnyCanvas::Locators(canvas) => canvas.draw(state, renderer, theme, bounds, cursor),
//             AnyCanvas::Vignette(canvas) => canvas.draw(state, renderer, theme, bounds, cursor),
//         }
//     }
// }
// 
// impl RootVisible for AnyCanvas {
//     fn view(&self) -> Element<RootMessage> {
//         canvas::Program::view(self)  // Delegate to Program::view
//     }
// }

impl VisualRoot {
    pub fn update(&mut self, message: RootMessage) -> Task<RootMessage> {
        match message {
            RootMessage::Exit => {
                window::get_latest().and_then(window::close)
            }
            RootMessage::LocatorsCanvas => {
                let locators = get_root_locators().expect("Can't get root locators");
                let locators_trie = LocatorTrieNode::new(locators);
                self.initiatedVisual = Some(AvailableVisible::LocatorsCanvas(LocatorsCanvas::new(locators_trie ,None)));
                Task::none()
            }
            RootMessage::Vignette => {
                self.initiatedVisual = Some(AvailableVisible::Vignette(VignetteCanvas { border_width: 20.0, opacity: 0.2 }));
                Task::none()
            }
        }

    }

    pub  fn view(&self) -> Element<'_, RootMessage> {
        if let Some(initiated_canvas) = &self.initiatedVisual {
            let initiated:Element<'_, RootMessage> = match initiated_canvas { 
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
            };
            
            return initiated;
        }

        iced::widget::text("Some cool text").into()
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
            // background_color: iced::Color::TRANSPARENT,
            background_color: iced::Color::from_rgb(0.7, 0.7, 0.7),
            text_color: theme.palette().text,
        }
    }
}
