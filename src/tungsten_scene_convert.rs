use std::collections::HashMap;

use embree;

use crate::math::*;
use crate::colour::*;
use crate::geometry::*;
use crate::materials::*;
use crate::material_type::*;

use crate::scene::SceneBuilder;
use crate::camera::Camera;
use crate::render_settings::{RenderSettings};

use crate::tungsten_scene;

impl tungsten_scene::Transform {
    pub fn to_affine_transform(&self) -> AffineTransform {
        let x = Matrix3::from_axis_angle(Vector3::unit_x(), Deg(self.rotation[0]));
        let y = Matrix3::from_axis_angle(Vector3::unit_y(), Deg(self.rotation[1]));
        let z = Matrix3::from_axis_angle(Vector3::unit_z(), Deg(self.rotation[2]));
        AffineTransform {
            rotation: y * x * z,
            scale: self.scale.into(),
            translation: Vector3::from(self.position),
        }
    }
}

const CUBE_VERTICES: [Point3<f32>; 8] = [
    Point3 { x: -0.5, y: -0.5, z: -0.5 },
    Point3 { x: -0.5, y: -0.5, z:  0.5 },
    Point3 { x: -0.5, y:  0.5, z: -0.5 },
    Point3 { x: -0.5, y:  0.5, z:  0.5 },
    Point3 { x:  0.5, y: -0.5, z: -0.5 },
    Point3 { x:  0.5, y: -0.5, z:  0.5 },
    Point3 { x:  0.5, y:  0.5, z: -0.5 },
    Point3 { x:  0.5, y:  0.5, z:  0.5 },
];

const CUBE_INDICES: [embree::Triangle; 12] = [
    // Left side
    embree::Triangle { v0: 0, v1: 1, v2: 2 },
    embree::Triangle { v0: 1, v1: 3, v2: 2 },
    // Right side
    embree::Triangle { v0: 4, v1: 6, v2: 5 },
    embree::Triangle { v0: 5, v1: 6, v2: 7 },
    // Bottom side
    embree::Triangle { v0: 0, v1: 4, v2: 1 },
    embree::Triangle { v0: 1, v1: 4, v2: 5 },
    // Top side
    embree::Triangle { v0: 2, v1: 3, v2: 6 },
    embree::Triangle { v0: 3, v1: 7, v2: 6 },
    // Front side
    embree::Triangle { v0: 0, v1: 2, v2: 4 },
    embree::Triangle { v0: 2, v1: 6, v2: 4 },
    // Back side
    embree::Triangle { v0: 1, v1: 5, v2: 3 },
    embree::Triangle { v0: 3, v1: 5, v2: 7 },
];

// Source: https://refractiveindex.info/
const _METAL_IOR: [(&str, &str, Ior); 3] = [
    ("Au", "Gold", Ior { n: [0.15557, 0.42415, 1.3831], k: [3.6024, 2.4721, 1.9155]}),
    ("Ag", "Silver", Ior { n: [0.052225, 0.059582, 0.040000], k: [4.4094, 3.5974, 2.6484]}),
    ("Cu", "Copper", Ior { n: [0.23780, 1.0066, 1.2404], k: [3.6264, 2.5823, 2.3929]}),
];

impl tungsten_scene::SceneDescription {
    pub fn build_camera(&self) -> Camera {
        let res = self.camera.resolution;
        let aspect = (res[0] as f32) / (res[1] as f32);
        let cam = &self.camera;
        Camera::new(Point3::from(cam.transform.position),
            Point3::from(cam.transform.look_at),
            Vector3::from(cam.transform.up),
            Deg(cam.fov_degrees),
            aspect)
    }

    pub fn resolution(&self) -> (u32, u32) {
        (self.camera.resolution[0], self.camera.resolution[1])
    }

    pub fn render_settings(&self) -> RenderSettings {
        RenderSettings {
            spp: self.renderer.spp,
            max_depth: self.integrator.max_bounces,
        }
    }

    pub fn add_primitives(&self, scene: &mut SceneBuilder) {
        let mut materials = HashMap::new();

        for bsdf in &self.bsdfs {
            let albedo = bsdf.albedo.into();
            #[allow(unreachable_patterns)]
            let m = match &bsdf.bsdf {
                tungsten_scene::BSDF::Lambert {} => {
                    MaterialType::Diffuse(Lambert::new(albedo))
                },
                tungsten_scene::BSDF::Null => MaterialType::Diffuse(Lambert::new(Colour::zero())),
                b => {
                    warn!("Unsupported BSDF type: {:?}", b);
                    MaterialType::Diffuse(Lambert::new(Colour::zero()))
                },
            };
            materials.insert(bsdf.name.clone(), m);
        }
        for prim in &self.primitives {
            let mat = materials.get(&prim.bsdf).expect("Undeclared material");

            let transform = prim.transform.to_affine_transform();
            let emission = prim.emission.map_or_else(Colour::zero, Into::into);

            #[allow(unreachable_patterns)]
            match &prim.primitive {
                tungsten_scene::PrimitiveType::Sphere => {
                    let mut sphere = Sphere::unit();
                    sphere.transform_by(&transform);
                    sphere.emission = emission;
                    scene.add_sphere(sphere, mat.clone());
                },
                tungsten_scene::PrimitiveType::Quad => {
                    let mut quad = Quad::new(
                        Point3::new(-0.5, 0.0, -0.5),
                        Point3::new( 0.5, 0.0, -0.5),
                        Point3::new( 0.5, 0.0,  0.5),
                        Point3::new(-0.5, 0.0,  0.5));
                    quad.transform_by(&transform);
                    quad.emission = emission;
                    scene.add_quad(quad, mat.clone());
                }
                tungsten_scene::PrimitiveType::Cube => {
                    let mut cube = embree::TriangleMesh::new(&scene.device,
                        Vec::from(CUBE_INDICES.as_ref()),
                        Vec::from(CUBE_VERTICES.as_ref()));
                    let matrix = transform.to_matrix();
                    cube.transform_mesh(matrix);
                    scene.add_mesh(cube, mat.clone());
                }
                t => warn!("Unknown primitive type: {:?}", t),
            }
        }
    }
}