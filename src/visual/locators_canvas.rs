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
        let children = locators_trie.children.clone().unwrap();
        let locations_paths = LocatorCanvas::filtered_children(children, location_key.clone());

        let root = locators_trie;
        
        LocatorCanvas {
            location_key,
            locations_paths,
            root,
        }
    }
    
    pub fn update(&mut self, location_key: Option<String>){
        let children = self.root.children.clone().unwrap();
        self.locations_paths = LocatorCanvas::filtered_children(children, location_key);
    }
    
    fn filtered_children(children: Vec<LocatorTrieNode>, location_key: Option<String>) -> Option<Vec<(LocatorTrieNode, String)>>{
        match location_key {
            Some(target_key) => Some(children
                .into_iter()
                .filter_map(|child| LocatorTrieNode::accessible_children(child, target_key.as_str()))
                .fold(vec![], |mut acc, children| {
                    acc.extend(children);
                    acc}
                )
                .into_iter()
                .collect()),
            None => Some(
                children
                    .into_iter()
                    .map(|child| child.get_children())
                    .fold(vec![], |mut acc, children| {acc.extend(children); acc})
                    .into_iter()
                    .collect(),
            )
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
