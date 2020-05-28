use crate::accel::aabb::{AxisAlignedBoundingBox, Boundable};
use crate::common::*;
use crate::geometry::Hitable;
use std::cmp::min;
use std::path::PathBuf;

pub struct TriangleMesh {
    //Same as tobj::Mesh
    pub positions: Vec<f32>,
    pub normals: Vec<f32>,
    pub texture_coordinates: Vec<f32>,
    pub indices: Vec<u32>,
    pub material_id: Option<usize>,
}

#[derive(Clone)]
pub struct Triangle {
    //TODO Visit this implementation someday after leveling up
    //indices: Vec<u128>,
    //mesh: Arc<TriangleMesh>,
    pub positions: Vec<Point3>,
    pub normals: Vec<Vec3>,
    pub texture_coordinates: Vec<Point2>,
    pub bounding_box: AxisAlignedBoundingBox,
}

impl TriangleMesh {
    pub fn new(mesh_name_and_path: PathBuf) -> Vec<TriangleMesh> {
        //Load in the .obj file. It might have multiple models(meshes) in it
        let obj_mesh = tobj::load_obj(mesh_name_and_path.as_path());
        assert!(obj_mesh.is_ok());
        let (models, materials) = obj_mesh.unwrap();
        let mut meshes: Vec<TriangleMesh> = Vec::new();
        for model in models {
            let mesh = TriangleMesh {
                positions: model.mesh.positions,
                normals: model.mesh.normals,
                texture_coordinates: model.mesh.texcoords,
                indices: model.mesh.indices,
                material_id: model.mesh.material_id,
            };
            meshes.push(mesh);
        }

        meshes
    }

    pub fn get_triangles_from_mesh(&self) -> Vec<Triangle> {
        let mut triangles: Vec<Triangle> = vec![];

        //*self.positions.get(index[0] as usize).unwrap() as fp
        //Convert the indices to groups of 3
        for v in 0..self.indices.len() / 3 {
            let index_0_of_triangle = self.indices[3 * v] as usize;
            let index_1_of_triangle = self.indices[3 * v + 1] as usize;
            let index_2_of_triangle = self.indices[3 * v + 2] as usize;

            let mut triangle = Triangle {
                positions: vec![
                    Point3::new(
                        fp::from(self.positions[3 * index_0_of_triangle]),
                        fp::from(self.positions[3 * index_0_of_triangle + 1]),
                        fp::from(self.positions[3 * index_0_of_triangle + 2]),
                    ),
                    Point3::new(
                        fp::from(self.positions[3 * index_1_of_triangle]),
                        fp::from(self.positions[3 * index_1_of_triangle + 1]),
                        fp::from(self.positions[3 * index_1_of_triangle + 2]),
                    ),
                    Point3::new(
                        fp::from(self.positions[3 * index_2_of_triangle]),
                        fp::from(self.positions[3 * index_2_of_triangle + 1]),
                        fp::from(self.positions[3 * index_2_of_triangle + 2]),
                    ),
                ],
                normals: vec![
                    Vector3::new(
                        fp::from(self.normals[3 * index_0_of_triangle]),
                        fp::from(self.normals[3 * index_0_of_triangle + 1]),
                        fp::from(self.normals[3 * index_0_of_triangle + 2]),
                    )
                    .normalize(),
                    Vector3::new(
                        fp::from(self.normals[3 * index_1_of_triangle]),
                        fp::from(self.normals[3 * index_1_of_triangle + 1]),
                        fp::from(self.normals[3 * index_1_of_triangle + 2]),
                    )
                    .normalize(),
                    Vector3::new(
                        fp::from(self.normals[3 * index_2_of_triangle]),
                        fp::from(self.normals[3 * index_2_of_triangle + 1]),
                        fp::from(self.normals[3 * index_2_of_triangle + 2]),
                    )
                    .normalize(),
                ],
                texture_coordinates: vec![
                    Point2::new(
                        fp::from(self.texture_coordinates[2 * index_0_of_triangle]),
                        fp::from(self.texture_coordinates[2 * index_0_of_triangle + 1]),
                    ),
                    Point2::new(
                        fp::from(self.texture_coordinates[2 * index_1_of_triangle]),
                        fp::from(self.texture_coordinates[2 * index_1_of_triangle + 1]),
                    ),
                    Point2::new(
                        fp::from(self.texture_coordinates[2 * index_2_of_triangle]),
                        fp::from(self.texture_coordinates[2 * index_2_of_triangle + 1]),
                    ),
                ],

                bounding_box: AxisAlignedBoundingBox::default(),
            };
            triangle.bounding_box = Triangle::set_bounding_box(&triangle);
            // info!("AABB of triangle: {:?}", triangle.bounding_box);
            /*
            warn!(" Positions of triangle {} : {} {} {}", v, index_0_of_triangle,
                  index_1_of_triangle, index_2_of_triangle);
            warn!("    v[{}] = ({}, {}, {})", v,  self.positions[3*index_0_of_triangle],
                  self.positions[3*index_0_of_triangle + 1], self.positions[3*index_0_of_triangle + 2]);
            warn!("    v[{}] = ({}, {}, {})", v,  self.positions[3*index_1_of_triangle],
                  self.positions[3*index_1_of_triangle + 1], self.positions[3*index_1_of_triangle + 2]);
            warn!("    v[{}] = ({}, {}, {})", v,  self.positions[3*index_2_of_triangle],
                  self.positions[3*index_2_of_triangle + 1], self.positions[3*index_2_of_triangle + 2]);
            */
            triangles.push(triangle);
        }

        //info!("Size of triangles: {}", triangles.len());
        triangles
    }
}

impl Hitable for Triangle {
    fn check_intersection_and_return_closest_hit(&self, ray: Ray) -> Option<IntersectionInfo> {
        //Follow pbrt's watertight ray-triangle intersection
        /*
        3-step transformation to transform the triangle and the ray to ray-triangle intersection coordinate system s.t. ray's origin is at (0,0,0):
        a translation, a permutation and a shear.
        */
        //1. Translate triangle
        let mut p0t: Point3 = self.positions[0] - ray.o;
        let mut p1t: Point3 = self.positions[1] - ray.o;
        let mut p2t: Point3 = self.positions[2] - ray.o;

        //2. Permute the vertices
        //Find max dimension to permute to
        let kz: i32 = ray.d.abs().max_dimension();
        let mut kx: i32 = kz + 1;
        if kx == 3 {
            kx = 0;
        }
        let mut ky: i32 = kx + 1;
        if ky == 3 {
            ky = 0;
        }
        //info!("Ray dimensions: {} {} {}", kx, ky, kz);
        //Permute the vertices
        let d: Vector3 = ray.d.permute(kx, ky, kz);
        p0t = p0t.permute(kx, ky, kz);
        p1t = p1t.permute(kx, ky, kz);
        p2t = p2t.permute(kx, ky, kz);

        //3. Shear
        //Only shear x and y dimensions
        let sx: fp = -d.x / d.z;
        let sy: fp = -d.y / d.z;
        let sz: fp = 1.0 / d.z;
        p0t.x += sx * p0t.z;
        p0t.y += sy * p0t.z;
        p1t.x += sx * p1t.z;
        p1t.y += sy * p1t.z;
        p2t.x += sx * p2t.z;
        p2t.y += sy * p2t.z;

        //info!("Trying to intersect ray o:{:?}, d:{:?} with triangle with positions: {:?} before:{:?}", ray.o, ray.d, p0t, self.positions[0] - ray.o);

        //4. Now compute if ray from (0,0) along +z axis intersects this transformed triangle.
        //Due to transformation, equivalent to determining if (0,0) is inside the xy-projection
        let e0: fp = p1t.x * p2t.y - p1t.y * p2t.x;
        let e1: fp = p2t.x * p0t.y - p2t.y * p0t.x;
        let e2: fp = p0t.x * p1t.y - p0t.y * p1t.x;

        if (e0 < 0.0 || e1 < 0.0 || e2 < 0.0) && (e0 > 0.0 || e1 > 0.0 || e2 > 0.0) {
            return None;
        }
        let det: fp = e0 + e1 + e2;
        if det == 0.0 {
            return None;
        }

        //5. Check if t value is valid
        //Compute t by interpolating z_i with e_i, and checking if sign of d=e0+e1+e2 and this
        //interpolated t are different. If yes, the final t will be negative and not valid intersection
        p0t.z *= sz;
        p1t.z *= sz;
        p2t.z *= sz;

        let t_scaled: fp = e0 * p0t.z + e1 * p1t.z + e2 * p2t.z;
        if det < 0.0 && (t_scaled >= 0.0 || t_scaled < ray.tmax * det) {
            return None;
        }
        if det > 0.0 && (t_scaled <= 0.0 || t_scaled > ray.tmax * det) {
            return None;
        }

        //6. Get t value and barycentric coordinates now that we are sure we have a valid intersection
        let inv_det: fp = 1.0 / det;
        let b0: fp = e0 * inv_det;
        let b1: fp = e1 * inv_det;
        let b2: fp = e2 * inv_det;
        let t: fp = t_scaled * inv_det;

        //7. Compute triangle partial derivatives for uv and hit point calculation
        //dpdu: Shading tangent

        let mut dpdu: Vector3 = Default::default();
        let mut dpdv: Vector3 = Default::default();
        let duv02: Vector2 = self.texture_coordinates[0] - self.texture_coordinates[2];
        let duv12: Vector2 = self.texture_coordinates[1] - self.texture_coordinates[2];
        let dp02: Vector3 = self.positions[0] - self.positions[2];
        let dp12: Vector3 = self.positions[1] - self.positions[2];

        let determinant: fp = duv02.x * duv12.y - duv02.y * duv12.x;
        if determinant == 0.0 {
            coordinate_system(
                (self.positions[2] - self.positions[0])
                    .cross(self.positions[1] - self.positions[0])
                    .normalize(),
                &mut dpdu,
                &mut dpdv,
            );
        } else {
            let inv_det_uv: fp = 1.0 / determinant;
            dpdu = (dp02 * duv12.y - dp12 * duv02.x) * inv_det_uv;
            dpdv = (dp02 * -duv12.y + dp12 * duv02.x) * inv_det_uv;
        }

        //8. Find point of intersection and texture coordinates at given point
        let p_hit: Point3 =
            self.positions[0] * b0 + self.positions[1] * b1 + self.positions[2] * b2;
        let uv_hit: Point2 = self.texture_coordinates[0] * b0
            + self.texture_coordinates[1] * b1
            + self.texture_coordinates[2] * b2;
        let mut geometric_normal: Vector3 = dp02.cross(dp12).normalize();
        geometric_normal.face_outward_normal(self.normals[0]);

        let intersection_info = IntersectionInfo {
            t_intersection: t,
            point_of_intersection: p_hit,
            normal: geometric_normal,
            is_aabb: false,
        };

        Some(intersection_info)
    }
}

impl Triangle {
    fn set_bounding_box(&self) -> AxisAlignedBoundingBox {
        // Bounding box for triangle = a box with minimum of all coordinates as one corner
        // and maximum of all coordinates as another corner
        //!("Calculating BB for Triangle with positions {:?} {:?} {:?}", self.positions[0], self.positions[1], self.positions[2]);
        let x_min: fp = fp::min(
            fp::min(self.positions[0].x, self.positions[1].x),
            self.positions[2].x,
        );
        let y_min: fp = fp::min(
            fp::min(self.positions[0].y, self.positions[1].y),
            self.positions[2].y,
        );
        let z_min: fp = fp::min(
            fp::min(self.positions[0].z, self.positions[1].z),
            self.positions[2].z,
        );
        let min_point: Point3 = Point3::new(x_min, y_min, z_min);

        let x_max: fp = fp::max(
            fp::max(self.positions[0].x, self.positions[1].x),
            self.positions[2].x,
        );
        let y_max: fp = fp::max(
            fp::max(self.positions[0].y, self.positions[1].y),
            self.positions[2].y,
        );
        let z_max: fp = fp::max(
            fp::max(self.positions[0].z, self.positions[1].z),
            self.positions[2].z,
        );
        let max_point: Point3 = Point3::new(x_max, y_max, z_max);
        //warn!("BB limits of triangle : {:?} {:?}", min_point, max_point);
        AxisAlignedBoundingBox::new_aabb(min_point, max_point)
    }
}
impl Boundable for Triangle {
    fn get_bounding_box(&self) -> AxisAlignedBoundingBox {
        self.bounding_box.clone()
    }
}
