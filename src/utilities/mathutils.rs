pub use cgmath::prelude::*;

pub use f64 as fp;
pub type Spectrum = cgmath::Vector3<fp>;
pub type Point3 = cgmath::Vector3<fp>;
pub type Point2 = cgmath::Vector2<fp>;
pub type Vec3 = cgmath::Vector3<fp>;
pub type Mat4 = cgmath::Matrix4<fp>;
pub const EPSILON: fp = 1e-5;
pub const ZERO_VEC3: cgmath::Vector3<fp> = cgmath::Vector3 {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};

pub const ZERO_VEC4: cgmath::Vector4<fp> = cgmath::Vector4 {
    x: 0.0,
    y: 0.0,
    z: 0.0,
    w: 0.0,
};

pub const ZERO_MAT4: cgmath::Matrix4<fp> = cgmath::Matrix4 {
    x: ZERO_VEC4,
    y: ZERO_VEC4,
    z: ZERO_VEC4,
    w: ZERO_VEC4,
};

