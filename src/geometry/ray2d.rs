use super::super::base_types::*;

pub struct Ray2D {
    pub origin: Vec2,
    pub direction: Vec2
}

impl Ray2D {
    pub fn new(origin: Vec2, direction: Vec2) -> Ray2D {
        Ray2D {origin, direction }
    }

    pub fn intersect(&self, other: &Ray2D) -> Option<Vec2> {
        let ao = self.origin.clone();
        let ad = self.direction.clone();
        let bo = other.origin.clone();
        let bd = other.direction.clone();

        let mut u: f64;
        let mut v: f64;
        if bd.0 != 0.0 {
            u = (ao.1*bd.0 + bd.1*bo.0 - bo.1*bd.0 - bd.1*ao.0 ) / (ad.0*bd.1 - ad.1*bd.0);
            v = (ao.0 + ad.0 * u - bo.0) / bd.0;
        } else if ad.0 != 0.0 {
            v = -((bo.1 - ao.1) * ad.0 + (ao.0 - bo.0) * ad.1) / (bd.1 * ad.0 + bd.0*ad.1);
            u = (bo.0 - ao.0 + v * bd.0) / ad.0;
        } else {
            return None;
        }

        if u >= 0.0 && v >= 0.0 {
            return Some(ao + ad * u);
        } else {
            return None;
        }
    }
}