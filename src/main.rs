mod base_types;

mod geometry {
    use super::base_types::*;

    pub struct Ray {
        origin: Vec2,
        direction: Vec2
    }
    
    pub enum GeometryObject {
        Segment { beg: Vec2, end: Vec2 },
        Circle { center: Vec2, radius: f64 },
        Arc { center: Vec2, radius: f64, }
    }
}

mod drawing {
    use super::geometry::*;
}


mod meshing {

}

extern crate piston_window;

use piston_window::*;

fn main() {

    let mut window: PistonWindow =
        WindowSettings::new("Finite Elements", [1000, 1000])
            .exit_on_esc(true).build().expect("Failed to create Window");
    
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {
            clear([0.1; 4], graphics);
            rectangle([0.0, 1.0, 0.0, 1.0],
                      [0.0, 0.0, 100.0, 100.0],
                      context.transform, graphics);
        });
    }
}
