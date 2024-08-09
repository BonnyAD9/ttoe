use std::{
    cmp::Ordering,
    fmt::Display,
    ops::{Add, AddAssign, Div, Index, IndexMut, Mul, Neg, Sub, SubAssign},
};

use crate::vec2_range::Vec2Range;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Vec2<T = usize> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn gt_or<I, R>(&self, rhs: I) -> bool
    where
        I: Into<Vec2<R>>,
        T: PartialOrd<R>,
    {
        let Vec2 { x, y } = rhs.into();
        self.x.partial_cmp(&x) == Some(Ordering::Greater)
            || self.y.partial_cmp(&y) == Some(Ordering::Greater)
    }

    pub fn cmul<I, R>(self, rhs: I) -> Vec2<T::Output>
    where
        T: Mul<R>,
        I: Into<Vec2<R>>,
    {
        let Vec2 { x, y } = rhs.into();
        Vec2::new(self.x * x, self.y * y)
    }
}

impl Vec2<usize> {
    pub fn wrapping_add_signed(
        &mut self,
        other: impl Into<Vec2<isize>>,
    ) -> Self {
        let Vec2 { x, y } = other.into();
        Self {
            x: self.x.wrapping_add_signed(x),
            y: self.y.wrapping_add_signed(y),
        }
    }

    pub fn clamp(&self, min: impl Into<Self>, max: impl Into<Self>) -> Self {
        let Self { x: minx, y: miny } = min.into();
        let Self { x: maxx, y: maxy } = max.into();
        Self::new(self.x.clamp(minx, maxx), self.y.clamp(miny, maxy))
    }

    pub fn saturating_sub(&self, other: impl Into<Self>) -> Self {
        let Self { x, y } = other.into();
        Self {
            x: self.x.saturating_sub(x),
            y: self.y.saturating_sub(y),
        }
    }

    pub fn to(self, other: Vec2) -> Vec2Range<usize> {
        Vec2Range::new(self, other)
    }
}

impl<T> Vec2<T>
where
    T: Mul<T>,
{
    pub fn prod(self) -> T::Output {
        self.x * self.y
    }
}

impl<T> From<(T, T)> for Vec2<T> {
    fn from((x, y): (T, T)) -> Self {
        Self::new(x, y)
    }
}

impl<T> From<[T; 2]> for Vec2<T> {
    fn from([x, y]: [T; 2]) -> Self {
        Self::new(x, y)
    }
}

impl<T> Index<usize> for Vec2<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            i => panic!("Index {i} is out of range of Vec2"),
        }
    }
}

impl<T> IndexMut<usize> for Vec2<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            i => panic!("Index {i} is out of range of Vec2"),
        }
    }
}

impl<L, R> Add<Vec2<R>> for Vec2<L>
where
    L: Add<R>,
{
    type Output = Vec2<L::Output>;

    fn add(self, rhs: Vec2<R>) -> Self::Output {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<L, R> AddAssign<Vec2<R>> for Vec2<L>
where
    L: AddAssign<R>,
{
    fn add_assign(&mut self, rhs: Vec2<R>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<L, R> Sub<Vec2<R>> for Vec2<L>
where
    L: Sub<R>,
{
    type Output = Vec2<L::Output>;

    fn sub(self, rhs: Vec2<R>) -> Self::Output {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<L, R> SubAssign<Vec2<R>> for Vec2<L>
where
    L: SubAssign<R>,
{
    fn sub_assign(&mut self, rhs: Vec2<R>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T> Neg for Vec2<T>
where
    T: Neg,
{
    type Output = Vec2<T::Output>;

    fn neg(self) -> Self::Output {
        Vec2::new(-self.x, -self.y)
    }
}

impl<L, R> Div<R> for Vec2<L>
where
    L: Div<R>,
    R: Copy,
{
    type Output = Vec2<L::Output>;

    fn div(self, rhs: R) -> Self::Output {
        Vec2::new(self.x / rhs, self.y / rhs)
    }
}

impl<T> Display for Vec2<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}
