use iced::{
    alignment::{Horizontal, Vertical},
    mouse,
    widget::{
        canvas::{self, Text},
        text::Shaping,
    },
    Color, Font, Point, Rectangle, Renderer, Theme,
};

use super::locators_trie_node::LocatorTrieNode;

pub struct LocatorCanvas<'a> {
    pub locators_trie: &'a LocatorTrieNode,
    pub location_key: Option<&'a str>,
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

        let locators_with_path = match &self.location_key {
            Some(target_key) => self.locators_trie.accessible_children(&target_key),
            None => self.locators_trie.get_children(),
        };

        locators_with_path.iter().for_each(|(node, id)| {
            let locator = node.node.as_ref().unwrap();
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
        });

        // // Test module
        // for i in (0..1300).step_by(10) {
        //     let text: Text = Text {
        //         content: format!("Coord: {:?}", i),
        //         position: Point::new(i as f32, i as f32),
        //         color: Color::from_rgb8(219, 174, 24),
        //         size: 10.0.into(), // Use appropriate size
        //         font: Font::default(),
        //         horizontal_alignment: Horizontal::Center,
        //         vertical_alignment: Vertical::Center,
        //         shaping: Shaping::Basic,

        //         ..Text::default()
        //     };
        //     frame.fill_text(text);
        // }

        vec![frame.into_geometry()]
    }
}
