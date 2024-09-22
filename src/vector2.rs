use eframe::egui::{Vec2, Vec2b};

use std::fmt::Display;

use std::ops::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vector2<T: Clone> {
    pub x: T,
    pub y: T,
}

pub type Vector2f = Vector2<f32>;

pub type Vector2d = Vector2<f64>;

pub type Vector2i = Vector2<i32>;

pub type Vector2b = Vector2<bool>;

impl<T, Rhs, W> Add<Vector2<Rhs>> for Vector2<T>
where
    T: Clone + Add<Rhs, Output = W>,
    Rhs: Clone,
    W: Clone,
{
    type Output = Vector2<W>;

    fn add(self, rhs: Vector2<Rhs>) -> Self::Output {
        Self::Output::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T, Rhs> AddAssign<Vector2<Rhs>> for Vector2<T>
where
    T: Clone + AddAssign<Rhs>,
    Rhs: Clone,
{
    fn add_assign(&mut self, rhs: Vector2<Rhs>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T, Rhs, W> Sub<Vector2<Rhs>> for Vector2<T>
where
    T: Clone + Sub<Rhs, Output = W>,
    Rhs: Clone,
    W: Clone,
{
    type Output = Vector2<W>;

    fn sub(self, rhs: Vector2<Rhs>) -> Self::Output {
        Self::Output::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T, Rhs> SubAssign<Vector2<Rhs>> for Vector2<T>
where
    T: Clone + SubAssign<Rhs>,
    Rhs: Clone,
{
    fn sub_assign(&mut self, rhs: Vector2<Rhs>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T, Rhs, W> Mul<Rhs> for Vector2<T>
where
    T: Clone + Mul<Rhs, Output = W>,
    Rhs: Copy,
    W: Clone,
{
    type Output = Vector2<W>;

    fn mul(self, rhs: Rhs) -> Self::Output {
        Self::Output::new(self.x * rhs, self.y * rhs)
    }
}

impl<T, Rhs> MulAssign<Rhs> for Vector2<T>
where
    T: Clone + MulAssign<Rhs>,
    Rhs: Copy,
{
    fn mul_assign(&mut self, rhs: Rhs) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T, Rhs, W> Div<Rhs> for Vector2<T>
where
    T: Clone + Div<Rhs, Output = W>,
    Rhs: Copy,
    W: Clone,
{
    type Output = Vector2<W>;

    fn div(self, rhs: Rhs) -> Self::Output {
        Self::Output::new(self.x / rhs, self.y / rhs)
    }
}

impl<T, Rhs> DivAssign<Rhs> for Vector2<T>
where
    T: Clone + DivAssign<Rhs>,
    Rhs: Copy,
{
    fn div_assign(&mut self, rhs: Rhs) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl<T: Clone + Default> Default for Vector2<T> {
    fn default() -> Self {
        Self::new(Default::default(), Default::default())
    }
}

impl<T: Clone + Display> Display for Vector2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T: Clone> Vector2<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Clone> From<Vector2<T>> for (T, T) {
    fn from(val: Vector2<T>) -> Self {
        (val.x, val.y)
    }
}

impl<T: Clone> From<Vector2<T>> for [T; 2] {
    fn from(val: Vector2<T>) -> Self {
        [val.x, val.y]
    }
}

impl<T: Clone> From<(T, T)> for Vector2<T> {
    fn from((x, y): (T, T)) -> Self {
        Self::new(x, y)
    }
}

impl<T: Clone> From<[T; 2]> for Vector2<T> {
    fn from([x, y]: [T; 2]) -> Self {
        Self::new(x, y)
    }
}

impl From<Vector2f> for Vec2 {
    fn from(val: Vector2f) -> Self {
        Self::new(val.x, val.y)
    }
}

impl From<Vec2> for Vector2f {
    fn from(value: Vec2) -> Self {
        Self::new(value.x, value.y)
    }
}

impl From<Vector2b> for Vec2b {
    fn from(val: Vector2b) -> Self {
        Self::new(val.x, val.y)
    }
}

impl From<Vec2b> for Vector2b {
    fn from(value: Vec2b) -> Self {
        Self::new(value.x, value.y)
    }
}
