use iced::{
    alignment::{Horizontal, Vertical},
    mouse::{self},
    widget::{
        canvas::{self, Text},
        text::Shaping,
    },
    Color, Font, Point, Rectangle, Renderer, Theme,
};

use super::locators_trie_node::LocatorTrieNode;

pub struct LocatorCanvas<'a> {
    pub location_key: Option<String>,
    pub locations_paths: Option<Vec<(&'a LocatorTrieNode, String)>>,
}

impl<'a> LocatorCanvas<'a> {
    pub fn new(locators_trie: LocatorTrieNode, location_key: Option<String>) -> LocatorCanvas<'a> {
        let locations_paths = match location_key.clone() {
            Some(target_key) => LocatorTrieNode::accessible_children(&locators_trie, &target_key),

            None => Some(locators_trie.get_children()),
        };

        LocatorCanvas {
            location_key,
            locations_paths,
        }
    }
}

impl<'a, Message> canvas::Program<Message> for LocatorCanvas<'a> {
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        println!("Key: {:?}", self.location_key);

        if let Some(locators_with_path) = &self.locations_paths {
            locators_with_path.iter().for_each(|(node, id)| {
                if let Some(locator) = node.node.as_ref() {
                    let text: Text = Text {
                        // IDK Do i need to bother about such cases(!!!) - i can expand key que if needed
                        content: format!("{:?}", id),
                        position: Point::new(
                            locator.resolution_point.x as f32,
                            (locator.resolution_point.y) as f32,
                        ),
                        color: Color::from_rgb8(219, 174, 24),
                        size: 15.0.into(), // Use appropriate size
                        font: Font::default(),
                        horizontal_alignment: Horizontal::Center,
                        vertical_alignment: Vertical::Center,
                        shaping: Shaping::Basic,

                        ..Text::default()
                    };
                    frame.fill_text(text);
                }
            });
        }

        vec![frame.into_geometry()]
    }
}
