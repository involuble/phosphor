use std::collections::HashMap;

use embree;

use math::*;
use colour::*;
use geometry::*;
use materials::*;
use material_type::*;

use scene::SceneBuilder;
use camera::Camera;
use render_settings::{RenderSettings};

use tungsten_scene;

fn val_to_colour(v: tungsten_scene::VectorValue) -> Colour {
    match v {
        tungsten_scene::VectorValue::Scalar(s) => Colour::new(s, s, s),
        tungsten_scene::VectorValue::Vector(v) => Colour::new(v[0], v[1], v[2]),
    }
}

fn val_to_vec3(v: tungsten_scene::VectorValue) -> Vector3<f32> {
    match v {
        tungsten_scene::VectorValue::Scalar(s) => Vector3::new(s, s, s),
        tungsten_scene::VectorValue::Vector(v) => Vector3::new(v[0], v[1], v[2]),
    }
}

impl tungsten_scene::Transform {
    pub fn to_affine_transform(&self) -> AffineTransform {
        let x = Matrix3::from_axis_angle(Vector3::unit_x(), Deg(self.rotation[0]));
        let y = Matrix3::from_axis_angle(Vector3::unit_y(), Deg(self.rotation[1]));
        let z = Matrix3::from_axis_angle(Vector3::unit_z(), Deg(self.rotation[2]));
        AffineTransform {
            rotation: y * x * z,
            scale: val_to_vec3(self.scale),
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
            let m = match bsdf.bsdf_type.as_ref() {
                "lambert" => {
                    MaterialType::Diffuse(Lambert::new(val_to_colour(bsdf.albedo)))
                },
                "null" => MaterialType::Diffuse(Lambert::new(Colour::zero())),
                b => {
                    warn!("Unknown BSDF type: {}", b);
                    MaterialType::Diffuse(Lambert::new(Colour::zero()))
                },
            };
            materials.insert(bsdf.name.clone(), m);
        }
        for prim in &self.primitives {
            let mat = materials.get(&prim.bsdf).expect("Undeclared material");

            let transform = prim.transform.to_affine_transform();
            let emission = if let Some(e) = prim.emission { val_to_colour(e) } else { Colour::zero() };

            match prim.primitive_type.as_ref() {
                "sphere" => {
                    let mut sphere = Sphere::unit();
                    sphere.transform_by(&transform);
                    sphere.emission = emission;
                    scene.add_sphere(sphere, mat.clone());
                },
                "quad" => {
                    let mut quad = Quad::new(
                        Point3::new(-0.5, 0.0, -0.5),
                        Point3::new( 0.5, 0.0, -0.5),
                        Point3::new( 0.5, 0.0,  0.5),
                        Point3::new(-0.5, 0.0,  0.5));
                    quad.transform_by(&transform);
                    quad.emission = emission;
                    scene.add_quad(quad, mat.clone());
                }
                "cube" => {
                    let mut cube = embree::TriangleMesh::new(&scene.device,
                        Vec::from(CUBE_INDICES.as_ref()),
                        Vec::from(CUBE_VERTICES.as_ref()));
                    let matrix = transform.to_matrix();
                    cube.transform_mesh(matrix);
                    scene.add_mesh(cube, mat.clone());
                }
                t => warn!("Unknown primitive type: {}", t),
            }
        }
    }
}