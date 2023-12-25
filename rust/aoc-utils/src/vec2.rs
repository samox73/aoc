use num_traits::Num;
use std::fmt::{Debug, Display};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vec2<T: Num> {
    pub x: T,
    pub y: T,
}

impl<T: Num + Copy> Vec2<T> {
    pub fn raw(&self) -> (T, T) {
        (self.x, self.y)
    }
}

impl<T: Num> std::ops::Add<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;
    fn add(self, other: Vec2<T>) -> Vec2<T> {
        Vec2::<T> {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Num + Copy> std::ops::Mul<T> for Vec2<T> {
    type Output = Vec2<T>;
    fn mul(self, other: T) -> Vec2<T> {
        Vec2::<T> {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl<T: Num + Copy> std::ops::AddAssign for Vec2<T> {
    fn add_assign(&mut self, rhs: Vec2<T>) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl<T: Num + Debug> Display for Vec2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:#?}, {:#?}]", self.x, self.y)
    }
}

impl<T: Num> From<(T, T)> for Vec2<T> {
    fn from(value: (T, T)) -> Self {
        Vec2::<T> {
            x: value.0,
            y: value.1,
        }
    }
}
