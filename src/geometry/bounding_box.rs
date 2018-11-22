use std::f64;
use std::ops::{Add, AddAssign};
use base_types::*;

#[derive(Debug)]
pub struct BoundingBox {
    pub l: f64, pub t: f64, pub r: f64, pub b:f64
}

impl BoundingBox {
    pub fn zero() -> BoundingBox {
        BoundingBox{l: 0.0, t: 0.0, r: 0.0, b: 0.0}
    }

    pub fn infinite() -> BoundingBox {
        BoundingBox {
            l: f64::NEG_INFINITY,
            t: f64::NEG_INFINITY,
            r: f64::INFINITY,
            b: f64::INFINITY
        }
    }

    pub fn null() -> BoundingBox {
        BoundingBox {
            l: f64::INFINITY,
            t: f64::INFINITY,
            r: f64::NEG_INFINITY,
            b: f64::NEG_INFINITY
        }
    }

    pub fn width(&self) -> f64 {
        self.r - self.l
    }

    pub fn height(&self) -> f64 {
        self.b - self.t
    }

    pub fn size(&self) -> Vec2 {
        Vec2(self.width(), self.height())
    }

    pub fn center_x(&self) -> f64 {
        (self.l + self.r) / 2.0
    }

    pub fn center_y(&self) -> f64 {
        (self.b + self.t) / 2.0
    }

    pub fn center(&self) -> Vec2 {
        Vec2(self.center_x(), self.center_y())
    }

    pub fn scale(&mut self, scale: f64) {
        *self = BoundingBox {
            l: self.l * scale,
            t: self.t * scale,
            r: self.r * scale,
            b: self.b * scale
        }
    }

}

impl<'a> Add<&'a Vec2> for BoundingBox {
    type Output = BoundingBox;
    fn add(self, other: &Vec2) -> BoundingBox {
        BoundingBox {
            l: self.l.min(other.0),
            t: self.t.min(other.1),
            r: self.r.max(other.0),
            b: self.b.max(other.1)
        }
    }
}

impl<'a> AddAssign<&'a Vec2> for BoundingBox {
    fn add_assign(&mut self, other: &Vec2) {
        *self = BoundingBox {
            l: self.l.min(other.0),
            t: self.t.min(other.1),
            r: self.r.max(other.0),
            b: self.b.max(other.1)
        }
    }
}

impl Add for BoundingBox {
    type Output = BoundingBox;
    fn add(self, other: BoundingBox) -> BoundingBox {
        BoundingBox {
            l: self.l.min(other.l),
            t: self.t.min(other.t),
            r: self.r.max(other.r),
            b: self.b.max(other.b)
        }
    }
}

impl AddAssign for BoundingBox {
    fn add_assign(&mut self, other: BoundingBox){
        *self = BoundingBox {
            l: self.l.min(other.l),
            t: self.t.min(other.t),
            r: self.r.max(other.r),
            b: self.b.max(other.b)
        }
    }
}