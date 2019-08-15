use crate::common::*;
use crate::geometry::Hitable;
use std::path::PathBuf;

pub struct TriangleMesh {
    //Same as tobj::Mesh
    pub positions: Vec<f32>,
    pub normals: Vec<f32>,
    pub texcoords: Vec<f32>,
    pub indices: Vec<u32>,
    pub material_id: Option<usize>,
}

pub struct Triangle {
    //TODO Visit this implementation someday after leveling up
    //indices: Vec<u128>,
    //mesh: Arc<TriangleMesh>,
    pub positions: Vec<Point3>,
    pub normals: Vec<Vec3>,
    pub texcoords: Vec<Point2>,
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
                texcoords: model.mesh.texcoords,
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

            let triangle = Triangle {
                positions: vec![
                    Point3::new(
                        self.positions[3 * index_0_of_triangle] as fp,
                        self.positions[3 * index_0_of_triangle + 1] as fp,
                        self.positions[3 * index_0_of_triangle + 2] as fp,
                    ),
                    Point3::new(
                        self.positions[3 * index_1_of_triangle] as fp,
                        self.positions[3 * index_1_of_triangle + 1] as fp,
                        self.positions[3 * index_1_of_triangle + 2] as fp,
                    ),
                    Point3::new(
                        self.positions[3 * index_2_of_triangle] as fp,
                        self.positions[3 * index_2_of_triangle + 1] as fp,
                        self.positions[3 * index_2_of_triangle + 2] as fp,
                    ),
                ],
                normals: vec![
                    Vector3::new(
                        self.normals[3 * index_0_of_triangle] as fp,
                        self.normals[3 * index_0_of_triangle + 1] as fp,
                        self.normals[3 * index_0_of_triangle + 2] as fp,
                    )
                    .normalize(),
                    Vector3::new(
                        self.normals[3 * index_1_of_triangle] as fp,
                        self.normals[3 * index_1_of_triangle + 1] as fp,
                        self.normals[3 * index_1_of_triangle + 2] as fp,
                    )
                    .normalize(),
                    Vector3::new(
                        self.normals[3 * index_2_of_triangle] as fp,
                        self.normals[3 * index_2_of_triangle + 1] as fp,
                        self.normals[3 * index_2_of_triangle + 2] as fp,
                    )
                    .normalize(),
                ],
                texcoords: vec![
                    Point2::new(
                        self.texcoords[2 * index_0_of_triangle] as fp,
                        self.texcoords[2 * index_0_of_triangle + 1] as fp,
                    ),
                    Point2::new(
                        self.texcoords[2 * index_1_of_triangle] as fp,
                        self.texcoords[2 * index_1_of_triangle + 1] as fp,
                    ),
                    Point2::new(
                        self.texcoords[2 * index_2_of_triangle] as fp,
                        self.texcoords[2 * index_2_of_triangle + 1] as fp,
                    ),
                ],
            };
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
        let mut ky: i32 = kz + 1;
        if ky == 3 {
            ky = 0;
        }
        //Permute the vertices
        p0t = p0t.permute(kx, ky, kz);
        p1t = p1t.permute(kx, ky, kz);
        p2t = p2t.permute(kx, ky, kz);

        //3. Shear
        //Only shear x and y dimensions
        let sx: fp = -ray.d.x / ray.d.z;
        let sy: fp = -ray.d.y / ray.d.z;
        let sz: fp = 1.0 / ray.d.z;
        p0t.x += sx * p0t.z;
        p0t.y += sy * p0t.z;
        p1t.x += sx * p1t.z;
        p1t.y += sy * p1t.z;
        p2t.x += sx * p2t.z;
        p2t.y += sy * p2t.z;

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
        } else if det > 0.0 && (t_scaled <= 0.0 || t_scaled > ray.tmax * det) {
            return None;
        }

        //6. Get t value and barycentric coordinates now that we are sure we have a valid intersection
        let inv_det: fp = 1.0 / det;
        let b0: fp = e0 * inv_det;
        let b1: fp = e1 * inv_det;
        let b2: fp = e2 * inv_det;
        let t: fp = t_scaled * inv_det;

        let intersection_info2 = IntersectionInfo {
            t_intersection: 0.0,
            point_of_intersection: Default::default(),
            normal: Vector3::from(t).normalize(),
        };
        return Some(intersection_info2);

        //7. Compute triangle partial derivatives for uv and hit point calculation
        //dpdu: Shading tangent

        let mut dpdu: Vector3 = Default::default();
        let mut dpdv: Vector3 = Default::default();
        let duv02: Vector2 = self.texcoords[0] - self.texcoords[2];
        let duv12: Vector2 = self.texcoords[1] - self.texcoords[2];
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
        let uv_hit: Point2 =
            self.texcoords[0] * b0 + self.texcoords[1] * b1 + self.texcoords[2] * b2;
        let mut geometric_normal: Vector3 = dp02.cross(dp12).normalize();
        geometric_normal.face_outward_normal(self.normals[0]);

        let intersection_info = IntersectionInfo {
            t_intersection: t,
            point_of_intersection: p_hit,
            normal: geometric_normal,
        };

        Some(intersection_info)
    }
}
