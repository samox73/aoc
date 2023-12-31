use num_traits::Num;
use std::fmt::{Debug, Display};

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Vec3<T: Num> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Num + Copy> Vec3<T> {
    pub fn raw(&self) -> (T, T, T) {
        (self.x, self.y, self.z)
    }
}

impl<T: Num> std::ops::Add<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;
    fn add(self, other: Vec3<T>) -> Vec3<T> {
        Vec3::<T> {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: Num + Copy> std::ops::Mul<T> for Vec3<T> {
    type Output = Vec3<T>;
    fn mul(self, other: T) -> Vec3<T> {
        Vec3::<T> {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl<T: Num + Copy> std::ops::AddAssign for Vec3<T> {
    fn add_assign(&mut self, rhs: Vec3<T>) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
    }
}

impl<T: Num + Debug> Display for Vec3<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:#?}, {:#?}, {:#?}]", self.x, self.y, self.z)
    }
}

impl<T: Num> From<(T, T, T)> for Vec3<T> {
    fn from(value: (T, T, T)) -> Self {
        Vec3::<T> {
            x: value.0,
            y: value.1,
            z: value.2,
        }
    }
}
