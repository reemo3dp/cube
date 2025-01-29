use std::ops::{Add, Sub};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Vector3d {
    pub x: i8,
    pub y: i8,
    pub z: i8,
}

impl Vector3d {
    pub fn min(self, other: Vector3d) -> Vector3d {
        Vector3d {
            x: std::cmp::min(self.x, other.x),
            y: std::cmp::min(self.y, other.y),
            z: std::cmp::min(self.z, other.z),
        }
    }
    pub fn max(self, other: Vector3d) -> Vector3d {
        Vector3d {
            x: std::cmp::max(self.x, other.x),
            y: std::cmp::max(self.y, other.y),
            z: std::cmp::max(self.z, other.z),
        }
    }
}

impl Sub<Vector3d> for Vector3d {
    type Output = Vector3d;

    fn sub(self, rhs: Vector3d) -> Self::Output {
        Vector3d {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Add<Vector3d> for Vector3d {
    type Output = Vector3d;

    fn add(self, rhs: Vector3d) -> Self::Output {
        Vector3d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
