use iced::gradient::ColorStop;
use iced::widget::canvas::gradient::Linear;
use iced::widget::canvas::{Frame, Geometry, Path};
use iced::{mouse::{self}, widget::canvas::{self}, Color, Point, Rectangle, Size};

#[derive(Clone)]
pub struct VignetteCanvas {
    pub border_width: f32,
    pub opacity: f32,
}

impl<'a, Message> canvas::Program<Message> for VignetteCanvas {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());

        let background = Path::rectangle(
            Point::new(0.0, 0.0),
            Size::new(bounds.width, bounds.height)
        );
        frame.fill(&background, Color::from_rgba(0.1, 0.1, 0.1, self.opacity));

        let side_height = bounds.height-(self.border_width * 2.0);
        let side_width = self.border_width;

        // Draw a single solid border to debug
        let top_border = Path::rectangle(
            Point::new(0.0, 0.0),
            Size::new(bounds.width, self.border_width)
        );
        let bottom_border = Path::rectangle(
            Point::new(0.0, bounds.height - self.border_width),
            Size::new(bounds.width, self.border_width)
        );
        let right_border = Path::rectangle(
            Point::new(bounds.width - self.border_width, self.border_width),
            Size::new(side_width, side_height)
        );
        let left_border = Path::rectangle(
            Point::new(0.0, self.border_width),
            Size::new(side_width, side_height)
        );

        let gradient = Linear::new(
            Point::new(400.0, 0.0),
            Point::new(bounds.width, self.border_width)
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