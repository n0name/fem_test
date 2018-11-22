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

            // let bb_rect = rectangle::Rectangle::new_border([1.0, 1.0, 0.0, 0.5], 0.5);
            // let temp = self.calc_bounding_box();
            // bb_rect.draw([temp.l, temp.t, temp.width(), temp.height()],  
            //     &draw_state::DrawState::new_alpha(), transform, g);
        }

        pub fn calc_bounding_box(&self) -> BoundingBox {
            self.objects.iter()
                .map(|o| o.bounding_box())
                .fold(BoundingBox::null(), |res, cur| res + cur)
        }
    }
}

mod meshing {
    // TODO
}

use piston_window::*;
use drawing::*;

use std::path::Path;

#[derive(Debug)]
struct Camera {
    pos: Vec2,
    scale: f64,
    speed: f64,
    pixel_size: Vec2
}

impl Camera {
    pub fn new(pixel_size: (f64, f64), speed: f64) -> Camera {
        Camera { pos: Vec2(0.0, 0.0), scale: 1.0, speed, 
        pixel_size: Vec2(pixel_size.0, pixel_size.1)}
    }

    pub fn center(&self, scale: f64) -> Vec2 {
        (self.pixel_size.clone() * scale) / 2.0 + self.pos.clone()
    }


    fn zoom_to_fit(&mut self, drw: &Drawing) {
        let mut bounding_box = drw.calc_bounding_box();
        self.scale = {
            let side = bounding_box.width().max(bounding_box.height());
            1000.0 / side
        };

        self.pos = Vec2( -bounding_box.l, -bounding_box.t);
        bounding_box.scale(self.scale);
        self.pos += (self.pixel_size.clone() - bounding_box.size()) / 2.0;
    }
}

fn main() {

    let file_name = Path::new(r#"D:\Temp\asdrcs.dxf"#);
    let objects = GeometryObject::read_from_file(file_name)
        .expect("Could not parse file");
    let drw = Drawing::from_obs(objects);

    let mut window: PistonWindow =
        WindowSettings::new("Finite Elements", [1000, 1000])
            .exit_on_esc(true).build().expect("Failed to create Window");


    window.set_lazy(true);
    let mut camera = Camera::new((f64::from(1000), f64::from(1000)), 3.0);

    // println!("Before zf: {:?}", camera);

    camera.zoom_to_fit(&drw);

    // println!("After zf: {:?}", camera);

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
                            Key::U => { camera.scale *= 1.1; camera.speed += 2.0 },
                            Key::J => { camera.scale *= 0.9; camera.speed -= 2.0 },
                            Key::F4 => {camera.zoom_to_fit(&drw)}
                            _ => ()
                        }
                    }
                }
            },
            _ => {
                window.draw_2d(&event, |context, graphics| {
                    clear([0.1; 4], graphics);
                    let tr = context.transform
                        .scale(camera.scale, camera.scale)
                        .trans(camera.pos.0, camera.pos.1);
                    drw.draw(tr, graphics);
                    println!("======================");
                });
            }

        }
    }

    // println!("End: {:?}", camera);
}
