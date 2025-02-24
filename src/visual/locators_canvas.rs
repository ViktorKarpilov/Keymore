use crate::{key_queue, locator::locator::Locator};
use iced::{
    alignment::{Horizontal, Vertical},
    mouse,
    widget::{
        canvas::{self, Text},
        text::Shaping,
    },
    Color, Font, Point, Rectangle, Renderer, Theme,
};

pub struct LocatorCanvas<'a> {
    pub locators: &'a Vec<Locator>,
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
        let mut keys = key_queue!();

        self.locators.iter().for_each(|locator| {
            // IDK Do i need to bother about such cases - i can expand key que if needed
            let text: Text = Text {
                content: format!("{:?}", keys.pop().unwrap_or("!!!").to_string()),
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
