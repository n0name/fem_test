pub mod ray2d;
pub use self::ray2d::Ray2D;
pub use super::base_types::*;

use std::f64;
use std::path::Path;
use piston_window::*;

use dxf::Drawing;
use dxf::entities::*;


#[derive(Debug)]
pub enum GeometryObject {
    Segment { beg: Vec2, end: Vec2 },
    Circle { center: Vec2, radius: f64 },
    Arc { center: Vec2, radius: f64, start: f64, sweep: f64},
    PolyLine { points: Vec<Vec2> }
}

impl From<&dxf::Point> for Vec2 {
    fn from(p: &dxf::Point) -> Self {
        Vec2(p.x, p.y)
    }
}

impl From<&dxf::Point> for Vec3 {
    fn from(p: &dxf::Point) -> Self {
        Vec3(p.x, p.y, p.z)
    }
}

trait OptionalFrom<T> {
    type Output;
    fn from_op(_: &T) -> Option<Self::Output>;
}

impl OptionalFrom<Entity> for GeometryObject {
    type Output = Self;
    fn from_op(entity: &Entity) -> Option<GeometryObject> {
        match entity.specific {
            EntityType::Circle(ref circle) => {
                Some(GeometryObject::Circle {
                    center: Vec2::from(&circle.center),
                    radius: circle.radius
                })
            },
            EntityType::Line(ref line) => {
                Some(GeometryObject::Segment {
                    beg: Vec2::from(&line.p1),
                    end: Vec2::from(&line.p2)
                })
            },
            EntityType::Arc(ref arc) => {
                Some(GeometryObject::Arc {
                    center: Vec2::from(&arc.center),
                    radius: arc.radius,
                    start: arc.start_angle,
                    sweep: arc.end_angle - arc.start_angle
                })
            },
            EntityType::Polyline(ref polyline) => {
                Some(GeometryObject::PolyLine {
                    points: polyline.vertices.iter()
                        .map(|v| Vec2::from(&v.location))
                        .collect()
                })
            }
            _ => None
        }
    }
}

use itertools::Itertools;

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
            GeometryObject::PolyLine {points} => {
                points.iter().tuple_windows::<(_, _)>()
                    .for_each(|(p1, p2)| {
                        line(color, 1.0,
                             [p1.0, p1.1, p2.0, p2.1],
                             transform, g)
                    });
            }
        }
    }

    pub fn read_from_file(file_name: &Path) -> Result<Vec<GeometryObject>, String> {
        let dxf_drawing = Drawing::load_file(file_name.to_str().unwrap()).unwrap();
        Ok(dxf_drawing.entities.iter()
            .map(|e| GeometryObject::from_op(e))
            .filter(|o| o.is_some())
            .map(|o| o.unwrap()) .collect())
    }
}