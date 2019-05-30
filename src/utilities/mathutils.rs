use core::ops;
pub use f64 as fp;
use std::cmp::max;

pub type Spectrum = Vector3;
pub type Point3 = Vector3;
pub type Point2 = Vector2;
pub type Vec3 = Vector3;

#[derive(Debug, Default, Clone, Copy)]
pub struct Vector3 {
    pub x: fp,
    pub y: fp,
    pub z: fp,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Vector2 {
    pub x: fp,
    pub y: fp,
}

impl ops::Sub for Vector3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Add for Vector3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Mul for Vector3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl ops::Div for Vector3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl ops::AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::SubAssign for Vector3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::MulAssign<fp> for Vector3 {
    fn mul_assign(&mut self, rhs: fp) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl ops::DivAssign<fp> for Vector3 {
    fn div_assign(&mut self, rhs: fp) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Vector3 {
    pub fn normalize(&self) -> Vector3 {
        let normalization_factor = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        Vector3 {
            x: self.x / normalization_factor,
            y: self.y / normalization_factor,
            z: self.z / normalization_factor,
        }
    }

    pub fn from(scalar: fp) -> Vector3 {
        Vector3 {
            x: scalar,
            y: scalar,
            z: scalar,
        }
    }

    pub fn new(scalar_x: fp, scalar_y: fp, scalar_z: fp) -> Vector3 {
        Vector3 {
            x: scalar_x,
            y: scalar_y,
            z: scalar_z,
        }
    }

    pub fn cross(&self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn max_dimension(&self) -> i32 {
        let max_dim;
        if self.x > self.y {
            if self.x > self.z {
                max_dim = 0;
            } else {
                max_dim = 2;
            }
        } else {
            if self.y > self.z {
                max_dim = 1;
            } else {
                max_dim = 2;
            }
        }
        max_dim
    }

    pub fn permute(&self, x: i32, y: i32, z: i32) -> Vector3 {
        let original_vector_elements: Vec<fp> = vec![self.x, self.y, self.z];

        Vector3 {
            x: *original_vector_elements.get(x as usize).unwrap(),
            y: *original_vector_elements.get(y as usize).unwrap(),
            z: *original_vector_elements.get(z as usize).unwrap(),
        }
    }
}

impl Vector2 {
    pub fn new(scalar_x: fp, scalar_y: fp) -> Vector2 {
        Vector2 {
            x: scalar_x,
            y: scalar_y,
        }
    }
}
