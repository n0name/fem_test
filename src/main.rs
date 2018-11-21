mod base_types;
mod geometry;

mod drawing {
    use super::geometry::*;

    pub struct Drawing {
        objects: Vec<GeometryObject>
    }
}


mod meshing {

}

extern crate piston_window;

use piston_window::*;
use base_types::*;
use geometry::*;

fn main() {
    let objects = vec![
        GeometryObject::Segment{beg: Vec2(50.0, 50.0), end: Vec2(100.0, 100.0)},
        GeometryObject::Circle{center: Vec2(200.0, 200.0), radius:100.0}
    ];

    let mut window: PistonWindow =
        WindowSettings::new("Finite Elements", [1000, 1000])
            .exit_on_esc(true).build().expect("Failed to create Window");
    
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {
            clear([0.1; 4], graphics);
            objects.iter().for_each(|o| o.draw(context.transform, graphics));
        });
    }
}
