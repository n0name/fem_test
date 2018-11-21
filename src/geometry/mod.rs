pub mod ray2d;
pub use self::ray2d::Ray2D;

pub use super::base_types::*;

pub enum GeometryObject {
    Segment { beg: Vec2, end: Vec2 },
    Circle { center: Vec2, radius: f64 },
    Arc { center: Vec2, radius: f64, start: f64, sweep: f64}
}

use piston_window::*;
use std::f64;

impl GeometryObject  {
    pub fn draw<G: Graphics>(&self, transform: math::Matrix2d, g: &mut G) {
        let color = [1.0, 1.0, 1.0, 1.0];
        match self {
            GeometryObject::Segment{beg, end} => {
                line(color, 1.0, 
                     [beg.0, beg.1, end.0, end.1], 
                     transform, g)
            },
            GeometryObject::Circle{center, radius} => {
                let r = [center.0 - radius,
                         center.1 - radius,
                         center.1 + radius,
                         center.0 + radius];

                let circ = ellipse::Ellipse::new_border(color, 1.0);
                circ.draw(r, &draw_state::DrawState::new_alpha(), transform, g)
            },
            GeometryObject::Arc{center, radius, start, sweep} => {
                let r = [center.0 - radius,
                         center.1 - radius,
                         center.1 + radius,
                         center.0 + radius];
                circle_arc(color, 1.0,
                           start - sweep, *start,
                           r, transform, g)
            },
        }
    }
}