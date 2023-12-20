use num_traits::Num;
use std::fmt::{Debug, Display};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Coordinate<T: Num> {
    pub x: T,
    pub y: T,
}

impl<T: Num + Copy> Coordinate<T> {
    fn raw(&self) -> (T, T) {
        (self.x, self.y)
    }
}

impl<T: Num> std::ops::Add<Coordinate<T>> for Coordinate<T> {
    type Output = Coordinate<T>;
    fn add(self, other: Coordinate<T>) -> Coordinate<T> {
        Coordinate::<T> {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Num + Copy> std::ops::Mul<T> for Coordinate<T> {
    type Output = Coordinate<T>;
    fn mul(self, other: T) -> Coordinate<T> {
        Coordinate::<T> {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl<T: Num + Copy> std::ops::AddAssign for Coordinate<T> {
    fn add_assign(&mut self, rhs: Coordinate<T>) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl<T: Num + Debug> Display for Coordinate<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:#?}, {:#?}]", self.x, self.y)
    }
}

impl<T: Num> From<(T, T)> for Coordinate<T> {
    fn from(value: (T, T)) -> Self {
        Coordinate::<T> {
            x: value.0,
            y: value.1,
        }
    }
}
