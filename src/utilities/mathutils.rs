use core::ops;
pub use f64 as fp;

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

impl ops::Mul<fp> for Vector3 {
    type Output = Self;
    fn mul(self, rhs: fp) -> Self::Output {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
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

impl ops::Div<fp> for Vector3 {
    type Output = Self;
    fn div(self, rhs: fp) -> Self::Output {
        Vector3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
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

    pub fn max_component_wise(&self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: if self.x >= rhs.x { self.x } else { rhs.x },
            y: if self.y >= rhs.y { self.y } else { rhs.y },
            z: if self.z >= rhs.z { self.z } else { rhs.z },
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

    pub fn dot(&self, b: Vector3) -> fp {
        self.x * b.x + self.y * b.y + self.z * b.z
    }

    pub fn face_outward_normal(&mut self, v: Vector3) {
        if self.dot(v) < 0.0 {
            *self *= -1.0;
        }
    }

    pub fn abs(&self) -> Vector3 {
        Vector3 {
            x: fp::abs(self.x),
            y: fp::abs(self.y),
            z: fp::abs(self.z),
        }
    }
}

//Utility function to construct a coordinate system by making a 2nd vector
//perpendicular to it and a 3rd vector perpendicular to both by cross product
pub fn coordinate_system(in_vec: Vector3, v2: &mut Vector3, v3: &mut Vector3) {
    let in_vec_normalized: Vector3 = in_vec.normalize();
    if fp::abs(in_vec_normalized.x) > fp::abs(in_vec_normalized.y) {
        *v2 = Vector3::new(-in_vec_normalized.z, 0.0, in_vec_normalized.x)
            / (fp::sqrt(
                in_vec_normalized.x * in_vec_normalized.x
                    + in_vec_normalized.z * in_vec_normalized.z,
            ));
    } else {
        *v2 = Vector3::new(0.0, in_vec_normalized.z, -in_vec_normalized.y)
            / (fp::sqrt(
                in_vec_normalized.y * in_vec_normalized.y
                    + in_vec_normalized.z * in_vec_normalized.z,
            ));
    }
    *v3 = in_vec_normalized.cross(v2.clone());
}

impl ops::Sub for Vector2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Add for Vector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Mul for Vector2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vector2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl ops::Mul<fp> for Vector2 {
    type Output = Self;
    fn mul(self, rhs: fp) -> Self::Output {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl ops::Div for Vector2 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Vector2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl ops::Div<fp> for Vector2 {
    type Output = Self;
    fn div(self, rhs: fp) -> Self::Output {
        Vector2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl ops::AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::SubAssign for Vector2 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::MulAssign<fp> for Vector2 {
    fn mul_assign(&mut self, rhs: fp) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl ops::DivAssign<fp> for Vector2 {
    fn div_assign(&mut self, rhs: fp) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl Vector2 {
    pub fn new(scalar_x: fp, scalar_y: fp) -> Vector2 {
        Vector2 {
            x: scalar_x,
            y: scalar_y,
        }
    }

    pub fn max_component_wise(&self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: if self.x >= rhs.x { self.x } else { rhs.x },
            y: if self.y >= rhs.y { self.y } else { rhs.y },
        }
    }
}
