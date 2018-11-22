use std::ops::{Add, Mul, Sub, Div, Neg};
use std::ops::{AddAssign, MulAssign, SubAssign, DivAssign};
use super::traits::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Vec2(pub f64, pub f64);
#[derive(Debug, PartialEq, Clone)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Dot for Vec2 {
    type Output = f64;
    fn dot(&self, other: &Vec2) -> f64 {
        self.0 * other.0 + self.1 * other.1
    }
}

impl Cross for Vec2 {
    type Output = f64;
    fn cross(&self, other: &Vec2) -> f64 {
        self.0 * other.1 - self.1 * other.0
    }
}

#[allow(dead_code)]
impl Vec2 {
    pub fn length(&self) -> f64 { self.dot(self) }

    pub fn normalized(&self) -> Vec2 {
        self.clone() / self.length()
    }

    pub fn normalize(&mut self) {
        *self /= self.length();
    }
}

impl Dot for Vec3 {
    type Output = f64;
    fn dot(&self, other: &Vec3) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }
}

impl Cross for Vec3 {
    type Output = Vec3;
    fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3( self.1 * other.2 - self.2 * other.1, 
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0 )
    }
}

#[allow(dead_code)]
impl Vec3 {

    pub fn length(&self) -> f64 { self.dot(self) }

    pub fn normalized(&self) -> Vec3 {
        self.clone() / self.length()
    }

    pub fn normalize(&mut self) {
        *self /= self.length();
    }
}

macro_rules! op_vec2 {
    ($nm: ident, $f_name: ident, $op: tt) => {
        impl $nm for Vec2 {
            type Output = Vec2;
            fn $f_name(self, other: Vec2) -> Vec2 {
                Vec2(self.0 $op other.0, self.1 $op other.1)
            }
        }

        impl $nm<f64> for Vec2 {
            type Output = Vec2;
            fn $f_name(self, other: f64) -> Vec2 {
                Vec2(self.0 $op other, self.1 $op other)
            }
        }
    };
}


macro_rules! op_vec2_as {
    ($nm: ident, $f_name: ident, $op: tt) => {
        impl $nm for Vec2 {
            fn $f_name(&mut self, other: Vec2) {
                let x = self.0 $op other.0;
                let y = self.1 $op other.1;
                *self = Vec2(x, y);
            }
        }

        impl $nm<f64> for Vec2 {
            fn $f_name(&mut self, other: f64){
                let x = self.0 $op other;
                let y = self.1 $op other;
                *self = Vec2(x, y);
            }
        }
    };
}

macro_rules! op_vec3 {
    ($nm: ident, $f_name: ident, $op: tt) => {
        impl $nm for Vec3 {
            type Output = Vec3;
            fn $f_name(self, other: Vec3) -> Vec3 {
                Vec3(self.0 $op other.0, self.1 $op other.1, self.2 $op other.2)
            }
        }

        impl $nm<f64> for Vec3 {
            type Output = Vec3;
            fn $f_name(self, other: f64) -> Vec3 {
                Vec3(self.0 $op other, self.1 $op other, self.2 $op other)
            }
        }
    };
}

macro_rules! op_vec3_as {
    ($nm: ident, $f_name: ident, $op: tt) => {
        impl $nm for Vec3 {
            fn $f_name(&mut self, other: Vec3) {
                let x = self.0 $op other.0;
                let y = self.1 $op other.1;
                let z = self.2 $op other.2;
                *self = Vec3(x, y, z);
            }
        }

        impl $nm<f64> for Vec3 {
            fn $f_name(&mut self, other: f64){
                let x = self.0 $op other;
                let y = self.1 $op other;
                let z = self.2 $op other;
                *self = Vec3(x, y, z);
            }
        }
    };
}


op_vec2!(Add, add, +);
op_vec2!(Mul, mul, *);
op_vec2!(Sub, sub, -);
op_vec2!(Div, div, /);

op_vec2_as!(AddAssign, add_assign, +);
op_vec2_as!(MulAssign, mul_assign, *);
op_vec2_as!(SubAssign, sub_assign, -);
op_vec2_as!(DivAssign, div_assign, /);

op_vec3!(Add, add, +);
op_vec3!(Mul, mul, *);
op_vec3!(Sub, sub, -);
op_vec3!(Div, div, /);

op_vec3_as!(AddAssign, add_assign, +);
op_vec3_as!(MulAssign, mul_assign, *);
op_vec3_as!(SubAssign, sub_assign, -);
op_vec3_as!(DivAssign, div_assign, /);

impl Neg for Vec2 {
    type Output = Vec2;
    fn neg(self) -> Vec2 { Vec2(-self.0, -self.1) }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 { Vec3(-self.0, -self.1, -self.2) }
}

#[cfg(test)]
mod test {
    use super::*;
    macro_rules! test {
        ($name: ident, $op:tt, $a: expr, $b: expr, $c: expr) => {
            #[test]
            fn $name () {
                let a = $a;
                let b = $b;
                assert_eq!(a $op b, $c);
            }
        };
    }

    macro_rules! test_as {
        ($name: ident, $op:tt, $a: expr, $b: expr, $c: expr) => {
            #[test]
            fn $name () {
                let mut a = $a;
                let b = $b;
                a $op b;
                assert_eq!(a, $c);
            }
        };
    }

    // Vec2 Tests
    test!(v2_add,      +, Vec2(0.0, 0.0), Vec2(1.0, 1.0), Vec2(1.0, 1.0));
    test!(v2_add_s,    +, Vec2(1.0, 2.0),            3.0, Vec2(4.0, 5.0));
    test!(v2_sub,      -, Vec2(0.0, 0.0), Vec2(1.0, 1.0), Vec2(-1.0, -1.0));
    test!(v2_sub_s,    -, Vec2(1.0, 2.0),            3.0, Vec2(-2.0, -1.0));
    test!(v2_mul,      *, Vec2(2.0, 2.0), Vec2(3.0, 4.0), Vec2(6.0, 8.0));
    test!(v2_mul_s,    *, Vec2(1.0, 2.0),            3.0, Vec2(3.0, 6.0));
    test!(v2_div,      /, Vec2(3.0, 4.0), Vec2(3.0, 2.0), Vec2(1.0, 2.0));
    test!(v2_div_s,    /, Vec2(4.0, 8.0),            2.0, Vec2(2.0, 4.0));

    test_as!(v2_add_as,      +=, Vec2(0.0, 0.0), Vec2(1.0, 1.0), Vec2(1.0, 1.0));
    test_as!(v2_add_as_s,    +=, Vec2(1.0, 2.0),            3.0, Vec2(4.0, 5.0));
    test_as!(v2_sub_as,      -=, Vec2(0.0, 0.0), Vec2(1.0, 1.0), Vec2(-1.0, -1.0));
    test_as!(v2_sub_as_s,    -=, Vec2(1.0, 2.0),            3.0, Vec2(-2.0, -1.0));
    test_as!(v2_mul_as,      *=, Vec2(2.0, 2.0), Vec2(3.0, 4.0), Vec2(6.0, 8.0));
    test_as!(v2_mul_as_s,    *=, Vec2(1.0, 2.0),            3.0, Vec2(3.0, 6.0));
    test_as!(v2_div_as,      /=, Vec2(3.0, 4.0), Vec2(3.0, 2.0), Vec2(1.0, 2.0));
    test_as!(v2_div_as_s,    /=, Vec2(4.0, 8.0),            2.0, Vec2(2.0, 4.0));

    // Vec3 Tests
    test!(v3_add,      +, Vec3(0.0, 0.0, 0.0), Vec3(1.0, 1.0, 1.0), Vec3(1.0, 1.0, 1.0));
    test!(v3_add_s,    +, Vec3(1.0, 2.0, 3.0),                 3.0, Vec3(4.0, 5.0, 6.0));
    test!(v3_sub,      -, Vec3(0.0, 0.0, 0.0), Vec3(1.0, 1.0, 1.0), Vec3(-1.0, -1.0, -1.0));
    test!(v3_sub_s,    -, Vec3(1.0, 2.0, 3.0),                 3.0, Vec3(-2.0, -1.0, 0.0));
    test!(v3_mul,      *, Vec3(2.0, 2.0, 2.0), Vec3(3.0, 4.0, 5.0), Vec3(6.0, 8.0, 10.0));
    test!(v3_mul_s,    *, Vec3(1.0, 2.0, 3.0),                 3.0, Vec3(3.0, 6.0, 9.0));
    test!(v3_div,      /, Vec3(3.0, 4.0, 5.0), Vec3(3.0, 2.0, 5.0), Vec3(1.0, 2.0, 1.0));
    test!(v3_div_s,    /, Vec3(4.0, 8.0, 10.0),                2.0, Vec3(2.0, 4.0, 5.0));

    test_as!(v3_add_as,      +=, Vec3(0.0, 0.0, 0.0), Vec3(1.0, 1.0, 1.0), Vec3(1.0, 1.0, 1.0));
    test_as!(v3_add_as_s,    +=, Vec3(1.0, 2.0, 3.0),                 3.0, Vec3(4.0, 5.0, 6.0));
    test_as!(v3_sub_as,      -=, Vec3(0.0, 0.0, 0.0), Vec3(1.0, 1.0, 1.0), Vec3(-1.0, -1.0, -1.0));
    test_as!(v3_sub_as_s,    -=, Vec3(1.0, 2.0, 3.0),                 3.0, Vec3(-2.0, -1.0, 0.0));
    test_as!(v3_mul_as,      *=, Vec3(2.0, 2.0, 2.0), Vec3(3.0, 4.0, 5.0), Vec3(6.0, 8.0, 10.0));
    test_as!(v3_mul_as_s,    *=, Vec3(1.0, 2.0, 3.0),                 3.0, Vec3(3.0, 6.0, 9.0));
    test_as!(v3_div_as,      /=, Vec3(3.0, 4.0, 5.0), Vec3(3.0, 2.0, 5.0), Vec3(1.0, 2.0, 1.0));
    test_as!(v3_div_as_s,    /=, Vec3(4.0, 8.0, 10.0),                2.0, Vec3(2.0, 4.0, 5.0));
}