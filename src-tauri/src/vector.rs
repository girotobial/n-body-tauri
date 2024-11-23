use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, PartialEq, Eq, Clone, Copy, serde::Serialize)]
pub struct Vector2<T>
where
    T: Copy + Clone + PartialEq,
{
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T>
where
    T: Copy + Clone + PartialEq,
{
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> Vector2<T>
where
    T: Copy + Clone + PartialEq + Mul<Output = T> + Add<Output = T> + Into<f64>,
{
    pub fn magnitude(&self) -> f64 {
        let squares: f64 = (self.x * self.x + self.y * self.y).into();
        squares.sqrt()
    }
}

impl<T> Sub for Vector2<T>
where
    T: Copy + Clone + PartialEq + Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T> Add for Vector2<T>
where
    T: Copy + Clone + PartialEq + Add<Output = T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl<T> Div<T> for Vector2<T>
where
    T: Copy + Clone + PartialEq + Div<Output = T>,
{
    type Output = Self;

    fn div(self, other: T) -> Self::Output {
        Self {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl<T> Mul<T> for Vector2<T>
where
    T: Copy + Clone + PartialEq + Mul<Output = T>,
{
    type Output = Self;
    fn mul(self, other: T) -> Self::Output {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Default for Vector2<f64> {
    fn default() -> Self {
        Self::new(0.0, 0.0)
    }
}

impl<T> std::fmt::Display for Vector2<T>
where
    T: Copy + Clone + PartialEq + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:.2}, {:.2})", self.x, self.y)
    }
}

impl<T> From<[T; 2]> for Vector2<T>
where
    T: Copy + Clone + PartialEq,
{
    fn from(value: [T; 2]) -> Self {
        Self {
            x: value[0],
            y: value[1],
        }
    }
}
