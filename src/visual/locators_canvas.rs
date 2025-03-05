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

#[derive(Clone)]
pub struct LocatorCanvas {
    pub location_key: Option<String>,
    pub locations_paths: Option<Vec<(LocatorTrieNode, String)>>,
    pub root: LocatorTrieNode,
}

impl LocatorCanvas {
    pub fn new(locators_trie: LocatorTrieNode, location_key: Option<String>) -> LocatorCanvas {
        let locations_paths =
            LocatorCanvas::filtered_children(locators_trie.clone(), location_key.clone());

        let root = locators_trie;

        LocatorCanvas {
            location_key,
            locations_paths,
            root,
        }
    }

    pub fn update(&mut self, location_key: Option<String>) {
        let target_paths = LocatorCanvas::filtered_children(self.root.clone(), location_key);
        self.locations_paths = {
            match target_paths {
                Some(value) => {
                    if value.len() == 0 {
                        None
                    } else {
                        Some(value)
                    }
                }
                None => None,
            }
        };
    }

    fn filtered_children(
        children_root: LocatorTrieNode,
        // children: Vec<LocatorTrieNode>,
        location_key: Option<String>,
    ) -> Option<Vec<(LocatorTrieNode, String)>> {
        match location_key {
            Some(target_key) => Some(
                LocatorTrieNode::accessible_children(children_root, target_key.as_str())
                    .into_iter()
                    .fold(vec![], |mut acc, children| {
                        acc.extend(children);
                        acc
                    })
                    .into_iter()
                    .collect(),
            ),
            None => Some(
                children_root
                    .get_children()
                    .into_iter()
                    .map(|child| child.0)
                    .map(|child| child.get_children())
                    .fold(vec![], |mut acc, children| {
                        acc.extend(children);
                        acc
                    })
                    .into_iter()
                    .collect(),
            ),
        }
    }
}

impl<'a, Message> canvas::Program<Message> for LocatorCanvas {
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
