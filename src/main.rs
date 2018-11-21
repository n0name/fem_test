extern crate piston_window;

mod base_types;
mod geometry;
mod drawing {
    pub use super::geometry::*;
    use piston_window::*;

    pub struct Drawing {
        objects: Vec<GeometryObject>
    }

    impl Drawing {
        pub fn new() -> Drawing {
            Drawing{objects: Vec::new()}
        }

        pub fn add_object(&mut self, obj: GeometryObject) {
            self.objects.push(obj)
        }

        pub fn draw<G: Graphics>(&self, transform: math::Matrix2d, g: &mut G) {
            self.objects.iter().for_each(|o| o.draw(transform, g));
        }
    }
}

mod meshing {
    // TODO
}

use piston_window::*;
use drawing::*;

fn main() {
    let mut drw = Drawing::new();
    drw.add_object(GeometryObject::Segment{beg: Vec2(50.0, 50.0), end: Vec2(100.0, 100.0)});
    drw.add_object(GeometryObject::Circle{center: Vec2(200.0, 200.0), radius:100.0});
    drw.add_object(GeometryObject::Arc{center: Vec2(400.0, 400.0), radius: 25.0, start: 0.0, sweep: 1.6});

    let mut window: PistonWindow =
        WindowSettings::new("Finite Elements", [1000, 1000])
            .exit_on_esc(true).build().expect("Failed to create Window");
    
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {
            clear([0.1; 4], graphics);
            drw.draw(context.transform, graphics);
        });
    }
}
