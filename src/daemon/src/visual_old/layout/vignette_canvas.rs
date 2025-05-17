use iced::gradient::ColorStop;
use iced::widget::canvas::gradient::Linear;
use iced::widget::canvas::{Fill, Frame, Geometry, Path};
use iced::{mouse::{self}, widget::canvas::{self}, Color, Point, Rectangle, Renderer, Size, Theme};
use iced::advanced::layout::Node;
use iced::advanced::{Layout, Overlay};
use iced::advanced::renderer::Style;
use iced::mouse::Cursor;
use crate::visual_old::layout::transparent_layout::Message;

#[derive(Clone)]
pub struct VignetteCanvas {
    pub border_width: f32,
    pub opacity: f32,
}

impl Overlay<Message, Theme, Renderer> for VignetteCanvas {
    fn layout(&mut self, renderer: &Renderer, bounds: Size) -> Node {
        todo!()
    }

    fn draw(&self, renderer: &mut Renderer, theme: &Theme, style: &Style, layout: Layout<'_>, cursor: Cursor) {
        let mut frame = Frame::new(renderer, layout.bounds().size());

        let background = Path::rectangle(
            Point::new(0.0, 0.0),
            Size::new(layout.bounds().width, layout.bounds().height)
        );
        frame.fill(&background, Color::from_rgba(0.1, 0.1, 0.1, self.opacity));

        let side_height = layout.bounds().height-(self.border_width * 2.0);
        let side_width = self.border_width;

        // Draw a single solid border to debug
        let top_border = Path::rectangle(
            Point::new(0.0, 0.0),
            Size::new(layout.bounds().width, self.border_width)
        );
        let bottom_border = Path::rectangle(
            Point::new(0.0, layout.bounds().height - self.border_width),
            Size::new(layout.bounds().width, self.border_width)
        );
        let right_border = Path::rectangle(
            Point::new(layout.bounds().width - self.border_width, self.border_width),
            Size::new(side_width, side_height)
        );
        let left_border = Path::rectangle(
            Point::new(0.0, self.border_width),
            Size::new(side_width, side_height)
        );

        let gradient = Linear::new(
            Point::new(400.0, 0.0),
            Point::new(layout.bounds().width, self.border_width)
        ).add_stops(
            vec![
                ColorStop {
                    offset: 0.0,
                    color: Color::from_rgba(1.0, 0.3, 0.2, self.opacity),  // Red-orange
                },
                ColorStop {
                    offset: 0.5,
                    color: Color::from_rgba(0.8, 0.4, 0.0, self.opacity),  // Orange
                },
                ColorStop {
                    offset: 1.0,
                    color: Color::from_rgba(1.0, 0.6, 0.0, self.opacity),  // Yellow-orange
                },
            ],
        );

        frame.fill(&top_border, gradient);
        frame.fill(&bottom_border, gradient);
        frame.fill(&right_border, gradient);
        frame.fill(&left_border, gradient);

        vec![frame.into_geometry()]
    }
}

impl<'a, Message> canvas::Program<Message> for VignetteCanvas {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {

    }
}