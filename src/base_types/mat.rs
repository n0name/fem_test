use std::ops::{Add, Mul, Sub, Div};
use std::ops::{AddAssign, MulAssign, SubAssign, DivAssign};
use std::ops::{Index, IndexMut};

use super::traits::*;
use super::{Vec2, Vec3};

pub struct Mat2 {
    data: [f64; 4]
}

pub struct Mat3 {
    data: [f64; 9]
}

impl Mat2 {
    pub fn new(data: [f64; 4]) -> Mat2 {
        Mat2 {data}
    }

    pub fn zero() -> Mat2 {
        Mat2 {data: [0.0; 4]}
    }

    pub fn identity() -> Mat2 {
        Mat2 {data: [1.0, 0.0, 0.0, 1.0]}
    }

    pub fn size() -> usize {
        return 4;
    }
}

impl Dot<Vec2> for Mat2 {
    type Output = Vec2;
    fn dot(&self, other: &Vec2) -> Vec2{
        Vec2 ( other.0 * self[0] + other.1 * self[1],
            other.0 * self[2] + other.1 * self[3])
    }
}


impl Mat3 {
    fn new(data: [f64; 9]) -> Mat3 { Mat3 {data} }
    fn zero() -> Mat3 {
        Mat3 {data: [0.0; 9]}
    }

    pub fn identity() -> Mat3 {
        Mat3 {data: [1.0, 0.0, 0.0, 
                     0.0, 1.0, 0.0,
                     0.0, 0.0, 1.0]}
    }
    pub fn size() -> usize {
        return 9;
    }
}

impl Dot<Vec3> for Mat3 {
    type Output = Vec3;
    fn dot(&self, other: &Vec3) -> Vec3{
        Vec3 ( other.0 * self[0] + other.1 * self[1] + other.2 * self[2],
               other.0 * self[3] + other.1 * self[4] + other.2 * self[5],
               other.0 * self[6] + other.1 * self[7] + other.2 * self[8])
    }
}


macro_rules! idx {
    ($obj: ident) => {
        impl Index<usize> for $obj {
            type Output = f64;
            fn index(&self, idx: usize) -> &f64 {
                debug_assert!(idx < $obj::size());
                &self.data[idx]
            }
        }

        impl IndexMut<usize> for $obj {
            fn index_mut<'a> (&'a mut self, idx: usize) -> &'a mut f64 {
                debug_assert!(idx < $obj::size());
                &mut self.data[idx]
            }
        }
    };
}

idx!(Mat2);
idx!(Mat3);

macro_rules! op {
    ($tr: ident, $fn: ident, $op: tt, $obj: ident) => {
        impl $tr for $obj {
            type Output = $obj;
            fn $fn(self, other: $obj) -> $obj {
                let mut res = $obj::zero();
                for i in 0..$obj::size() {
                    res[i] = self[i] $op other[i];
                }
                return res;
            }
        }

        impl $tr<f64> for $obj {
            type Output = $obj;
            fn $fn(self, other: f64) -> $obj {
                let mut res = $obj::zero();
                for i in 0..$obj::size() {
                    res[i] = self[i] $op other;
                }
                return res;
            }
        }
    };
}

op!(Add, add, +, Mat2);
op!(Sub, sub, -, Mat2);
op!(Mul, mul, *, Mat2);
op!(Div, div, /, Mat2);

op!(Add, add, +, Mat3);
op!(Sub, sub, -, Mat3);
op!(Mul, mul, *, Mat3);
op!(Div, div, /, Mat3);