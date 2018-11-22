pub mod ray2d;
pub mod bounding_box;

pub use base_types::*;
pub use self::ray2d::Ray2D;
pub use self::bounding_box::*;

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

impl<'a> From<&'a dxf::Point> for Vec2 {
    fn from(p: &dxf::Point) -> Self {
        Vec2(p.x, p.y)
    }
}

impl<'a> From<&'a dxf::Point> for Vec3 {
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
                    start: arc.start_angle.to_radians(),
                    sweep: (arc.end_angle - arc.start_angle).to_radians()
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
                         2.0 * radius, 2.0 * radius];

                let circ = ellipse::Ellipse::new_border(color, 1.0);
                circ.draw(r, &draw_state::DrawState::new_alpha(), transform, g)
            },
            GeometryObject::Arc{center, radius, start, sweep} => {
                let r = [center.0 - radius,
                         center.1 - radius,
                         2.0 * radius, 2.0 * radius];


                let clr = if *sweep > 0.0 { 
                    [0.2, 1.0, 0.8, 1.0]
                } else {
                    [0.8, 1.0, 0.2, 1.0]
                };

                circle_arc(clr, 1.0,
                           *start, start + sweep,
                           r, transform, g);

                let bb_rect = rectangle::Rectangle::new_border([1.0, 0.0, 0.0, 1.0], 1.0);
                let temp = self.bounding_box();
                bb_rect.draw([temp.l, temp.t, temp.width(), temp.height()],  
                    &draw_state::DrawState::new_alpha(), transform, g);

                let beg = Vec2::from_angle(*start) * (*radius) + center.clone();
                let circ = ellipse::Ellipse::new_border([1.0, 0.0, 1.0, 1.0], 1.0);
                circ.draw([beg.0 - 1.0, beg.1 - 1.0, 2.0, 2.0]
                    , &draw_state::DrawState::new_alpha(), transform, g)
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

    pub fn bounding_box(&self) -> BoundingBox {
        let mut bb = BoundingBox::null();
        match self {
            GeometryObject::Segment{ beg, end } => {
                bb += beg; bb += end;
            },
            GeometryObject::Circle{ center, radius } => {
                bb += &Vec2(center.0 - radius, center.1 - radius);
                bb += &Vec2(center.0 + radius, center.1 + radius);
            },
            GeometryObject::Arc{ center, radius, start, sweep } => {
                
                let r = *radius;

                let beg = Vec2::from_angle(*start);
                let end = Vec2::from_angle(start + sweep);
                let ox = Vec2::ox();
                let oy = Vec2::oy();

                bb += &(center.clone() + beg.clone() * r);
                bb += &(center.clone() + end.clone() * r);

                // let pi_2 = f64::consts::PI / 2.0;
                let two_pi = 2.0 * f64::consts::PI;


                let tmp_sweep = beg.dot(&end).acos().round().to_degrees() as i32;

                // let tmp_sweep = if *sweep < 0.0 {
                //     (two_pi + *sweep).to_degrees() as i32
                // } else {
                //     sweep.to_degrees() as i32
                // };

                let tmp_start = if *start < 0.0 {
                    (two_pi + *start).to_degrees() as i32
                } else {
                    start.to_degrees() as i32
                };

                let cnt = tmp_sweep / 90;
                let idx  =  tmp_start / 90;

                println!("{:?}. {:?}", tmp_start, tmp_sweep);
                println!("{:?}: {:?}, {:?}", center, idx, cnt);
                for i in 0..cnt {
                    match (idx + i) % 4 {
                        0 => {bb.r = center.0 + radius },
                        1 => {bb.t = center.1 - radius },
                        2 => {bb.l = center.0 - radius },
                        3 => {bb.b = center.1 + radius },
                        _ => {println!("Holy Shit !!")}
                    }
                }


                // let beg_ox = beg.dot(&ox);
                // let beg_oy = beg.dot(&oy);
                // let end_ox = end.dot(&ox);
                // let end_oy = end.dot(&oy);

                // if beg_ox > 0.0 && end_ox > 0.0 {
                //     bb.l = center.0 - radius;
                // } else if beg_ox < 0.0 && end_ox < 0.0 {
                //     bb.r = center.0 + radius;
                // } else {
                //     if beg_oy > 0.0 && end_oy > 0.0 {
                //         bb.b = center.1 + radius;
                //     } else if beg_oy < 0.0 && end_oy < 0.0 {
                //         bb.t = center.1 - radius;
                //     } else {
                //         bb += &Vec2(center.0 - radius, center.1 - radius);
                //         bb += &Vec2(center.0 + radius, center.1 + radius);
                //     }
                // }
            }
            GeometryObject::PolyLine{ points } => {
                points.iter().for_each(|p| bb+= p);
            }
        }
        return bb;
    }

    pub fn read_from_file(file_name: &Path) -> Result<Vec<GeometryObject>, String> {
        let dxf_drawing = Drawing::load_file(file_name.to_str().unwrap()).unwrap();
        Ok(dxf_drawing.entities.iter()
            .map(|e| GeometryObject::from_op(e))
            .filter(|o| o.is_some())
            .map(|o| o.unwrap()) .collect())
    }
}