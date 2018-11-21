pub trait Dot<T = Self> {
    type Output;
    fn dot(&self, other: &T) -> Self::Output;
}

pub trait Cross<T = Self> {
    type Output;
    fn cross(&self, other: &T) -> Self::Output;
}
