extern crate piston_window;
extern crate dxf;
extern crate itertools;

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
        pub fn from_obs(objects: Vec<GeometryObject>) -> Drawing {
            Drawing {objects}
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

use std::path::Path;

struct Camera {
    pos: Vec2,
    scale: f64,
    speed: f64
}

fn main() {

    let file_name = Path::new(r#"D:\Temp\Gear Sample-iss4\Gear Sample-iss4.DXF"#);
    let objects = GeometryObject::read_from_file(file_name)
        .expect("Could not parse file");
    let drw = Drawing::from_obs(objects);

    let mut window: PistonWindow =
        WindowSettings::new("Finite Elements", [1000, 1000])
            .exit_on_esc(true).build().expect("Failed to create Window");

    let mut camera = Camera{ pos: Vec2(0.0, 0.0), scale: 1.0, speed: 3.0 };

    while let Some(event) = window.next() {

        match event {
            Event::Input(input) => {
                if let Input::Button(btn) = input {
                    if let Button::Keyboard(key) = btn.button {
                        match key {
                            Key::W => { camera.pos += Vec2(0.0, -1.0) * camera.speed },
                            Key::A => { camera.pos += Vec2(-1.0, 0.0) * camera.speed },
                            Key::S => { camera.pos += Vec2(0.0, 1.0) * camera.speed },
                            Key::D => { camera.pos += Vec2(1.0, 0.0) * camera.speed },
                            Key::U => { camera.scale *= 1.1 },
                            Key::J => { camera.scale /= 0.9 },
                            _ => ()
                        }
                    }
                }
            },
            _ => {
                window.draw_2d(&event, |context, graphics| {
                    clear([0.1; 4], graphics);
                    let tr = context.transform
                        .trans(camera.pos.0, camera.pos.1)
                        .scale(camera.scale, camera.scale);
                    drw.draw(tr, graphics);
                });
            }

        }

    }
}
