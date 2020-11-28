use glam::*;
use embree;
use embree::{BuildQuality, SceneFlags, RayHit, Hit, GeomID};
use vec_map::VecMap;
use std::collections::HashMap;

use scene_import::SceneDescription;

use crate::math::*;
use crate::colour::*;
use crate::materials::*;
use crate::geometry::*;
use crate::geometry::{Sphere};

pub struct Scene {
    scene: embree::Scene,
    primitives: VecMap<Primitive>,
    skybox: Colour,
    pub lights: Vec<(GeomID, Box<dyn SampleableEmitter>)>,
}

#[derive(Debug, Clone)]
pub enum MaterialType {
    Diffuse(Lambert),
    // Null,
}

struct Primitive {
    pub emitter: EmissiveGeometry,
    pub material: MaterialType,
    // pub tex_scale: Vec2,
    // pub normal_map: Texture,
}

impl Primitive {
    pub fn new(material: MaterialType) -> Self {
        Primitive {
            emitter: EmissiveGeometry::NotEmissive,
            material: material,
        }
    }
}

#[derive(Clone)]
// TODO: private
pub enum EmissiveGeometry {
    NotEmissive,
    Sphere(Sphere),
    Quad(Quad),
}

pub struct ShadingParameters {
    pub basis: TangentFrame,
}

impl Scene {
    pub fn intersect(&self, rayhit: &mut RayHit) -> bool {
        self.scene.intersect(rayhit);
        rayhit.hit.Ng = rayhit.hit.Ng.normalize();
        rayhit.hit.is_hit()
    }

    pub fn occluded(&self, ray: &mut embree::Ray) -> bool {
        self.scene.occluded(ray)
    }

    pub fn skybox_emission(&self, _dir: Vec3) -> Colour {
        self.skybox
    }

    pub fn emission_at(&self, ray: &Ray, hit: &Hit) -> LightSample {
        let p = ray.point_at_dist(ray.tfar);
        let e = &self.primitives[hit.geom_id.unwrap() as usize].emitter;
        match e {
            EmissiveGeometry::NotEmissive => LightSample {
                dir: Vec3::zero(),
                distance: 0.0,
                radiance: Colour::zero(),
                pdf: PdfW(1.0),
            },
            EmissiveGeometry::Sphere(s) => s.eval_emission_at(ray.origin, p),
            EmissiveGeometry::Quad(q) => q.eval_emission_at(ray.origin, p),
        }
    }

    pub fn bsdf_at(&self, hit: &Hit) -> impl Bsdf {
        debug_assert!(!hit.geom_id.is_invalid());
        match self.primitives[hit.geom_id.id as usize].material {
            MaterialType::Diffuse(ref l) => l.clone(),
        }
    }
}

fn to_affine_transform(transform: &scene_import::Transform) -> AffineTransform {
    let x = Mat3::from_axis_angle(Vec3::unit_x(), transform.rotation[0] * PI / 180.0);
    let y = Mat3::from_axis_angle(Vec3::unit_y(), transform.rotation[1] * PI / 180.0);
    let z = Mat3::from_axis_angle(Vec3::unit_z(), transform.rotation[2] * PI / 180.0);
    AffineTransform {
        rotation: y * x * z,
        scale: transform.scale.into(),
        translation: transform.position.into(),
    }
}

const CUBE_VERTICES: [Vec3; 8] = [
    Vec3 { x: -0.5, y: -0.5, z: -0.5 },
    Vec3 { x: -0.5, y: -0.5, z:  0.5 },
    Vec3 { x: -0.5, y:  0.5, z: -0.5 },
    Vec3 { x: -0.5, y:  0.5, z:  0.5 },
    Vec3 { x:  0.5, y: -0.5, z: -0.5 },
    Vec3 { x:  0.5, y: -0.5, z:  0.5 },
    Vec3 { x:  0.5, y:  0.5, z: -0.5 },
    Vec3 { x:  0.5, y:  0.5, z:  0.5 },
];

const CUBE_INDICES: [embree::IndexedTriangle; 12] = [
    // Left side
    embree::IndexedTriangle { v0: 0, v1: 1, v2: 2 },
    embree::IndexedTriangle { v0: 1, v1: 3, v2: 2 },
    // Right side
    embree::IndexedTriangle { v0: 4, v1: 6, v2: 5 },
    embree::IndexedTriangle { v0: 5, v1: 6, v2: 7 },
    // Bottom side
    embree::IndexedTriangle { v0: 0, v1: 4, v2: 1 },
    embree::IndexedTriangle { v0: 1, v1: 4, v2: 5 },
    // Top side
    embree::IndexedTriangle { v0: 2, v1: 3, v2: 6 },
    embree::IndexedTriangle { v0: 3, v1: 7, v2: 6 },
    // Front side
    embree::IndexedTriangle { v0: 0, v1: 2, v2: 4 },
    embree::IndexedTriangle { v0: 2, v1: 6, v2: 4 },
    // Back side
    embree::IndexedTriangle { v0: 1, v1: 5, v2: 3 },
    embree::IndexedTriangle { v0: 3, v1: 5, v2: 7 },
];

// Source: https://refractiveindex.info/
// const _METAL_IOR: [(&str, &str, Ior); 3] = [
//     ("Au", "Gold", Ior { n: [0.15557, 0.42415, 1.3831], k: [3.6024, 2.4721, 1.9155]}),
//     ("Ag", "Silver", Ior { n: [0.052225, 0.059582, 0.040000], k: [4.4094, 3.5974, 2.6484]}),
//     ("Cu", "Copper", Ior { n: [0.23780, 1.0066, 1.2404], k: [3.6264, 2.5823, 2.3929]}),
// ];

pub struct SceneBuilder {
    pub device: embree::Device,
    scene: embree::SceneBuilder,
    primitives: VecMap<Primitive>,
    skybox: Colour,
    lights: Vec<(GeomID, Box<dyn SampleableEmitter>)>,
}

impl SceneBuilder {
    pub fn new(device: &embree::Device) -> Self {
        let mut s = embree::SceneBuilder::new(device);
        s.set_build_quality(BuildQuality::High);
        s.set_flags(SceneFlags::ROBUST | SceneFlags::COMPACT);

        SceneBuilder {
            device: device.clone(),
            skybox: Colour::zero(),
            scene: s,
            primitives: VecMap::new(),
            lights: Vec::new(),
        }
    }

    pub fn load_scene(&mut self, scene: &SceneDescription) {
        // TODO: do the hashmap  stuff in scene_desc
        let mut materials = HashMap::new();

        for mat in &scene.bsdfs {
            let albedo = mat.albedo.into();
            #[allow(unreachable_patterns)]
            let m = match &mat.bsdf {
                scene_import::MaterialType::Lambert {} => {
                    MaterialType::Diffuse(Lambert::new(albedo))
                },
                scene_import::MaterialType::Null => MaterialType::Diffuse(Lambert::new(Colour::zero())),
                b => {
                    log::warn!("Unsupported BSDF type: {:?}", b);
                    MaterialType::Diffuse(Lambert::new(Colour::zero()))
                },
            };
            materials.insert(mat.name.clone(), m);
        }
        for prim in &scene.primitives {
            let mat = materials.get(&prim.bsdf).expect("Undeclared material");

            let transform = to_affine_transform(&prim.transform);
            let emission = prim.emission.map_or_else(Colour::zero, Into::into);

            #[allow(unreachable_patterns)]
            match &prim.primitive {
                scene_import::PrimitiveType::Sphere => {
                    let mut sphere = Sphere::unit();
                    sphere.transform_by(&transform);
                    sphere.emission = emission;
                    self.add_sphere(sphere, mat.clone());
                },
                scene_import::PrimitiveType::Quad => {
                    let mut quad = Quad::new(
                        Vec3::new(-0.5, 0.0, -0.5),
                        Vec3::new( 0.5, 0.0, -0.5),
                        Vec3::new( 0.5, 0.0,  0.5),
                        Vec3::new(-0.5, 0.0,  0.5));
                    quad.transform_by(&transform);
                    quad.emission = emission;
                    self.add_quad(quad, mat.clone());
                }
                scene_import::PrimitiveType::Cube => {
                    let mut cube = embree::TriangleMesh::new(&self.device,
                        Vec::from(CUBE_INDICES.as_ref()),
                        Vec::from(CUBE_VERTICES.as_ref()));
                    let matrix = transform.to_matrix();
                    cube.transform_mesh(matrix);
                    self.add_mesh(cube, mat.clone());
                }
                t => log::warn!("Unknown primitive type: {:?}", t),
            }
        }
    }

    pub fn add_sphere(&mut self, sphere: Sphere, material: MaterialType) {
        let emitter;
        if sphere.is_emissive() {
            emitter = EmissiveGeometry::Sphere(sphere.clone());
        } else {
            emitter = EmissiveGeometry::NotEmissive;
        }
        let prim = Primitive {
            emitter: emitter,
            material: material,
        };

        // This seems to be slower than a user geometry, so keep using that
        // let embree_sphere = embree::Sphere {
        //     center: sphere.center,
        //     radius: sphere.radius,
        // };
        // let spheres = embree::SphereGeometry::new(&self.device, vec![embree_sphere]);
        // let id = self.scene.attach(spheres.build());

        let user = embree::UserGeometry::new(&self.device, vec![sphere.clone()]);
        let id = self.scene.attach(user);

        self.primitives.insert(id.unwrap() as usize, prim);
        if sphere.is_emissive() {
            self.lights.push((id, Box::new(sphere.clone())));
        }
    }

    pub fn add_quad(&mut self, quad: Quad, material: MaterialType) {
        let emitter;
        if quad.is_emissive() {
            emitter = EmissiveGeometry::Quad(quad.clone());
        } else {
            emitter = EmissiveGeometry::NotEmissive;
        }
        let prim = Primitive {
            emitter: emitter,
            material: material,
        };

        let index = vec![embree::IndexedTriangle::new(0, 1, 2), embree::IndexedTriangle::new(0, 2, 3)];
        let mesh = embree::TriangleMesh::new(&self.device, index, Vec::from(quad.points().as_ref()));
        let id = self.scene.attach(mesh);
        self.primitives.insert(id.unwrap() as usize, prim);
        if quad.is_emissive() {
            self.lights.push((id, Box::new(quad)));
        }
    }

    pub fn add_mesh(&mut self, mesh: embree::TriangleMesh, material: MaterialType) {
        // TODO: Maybe shouldn't expose embree API like this
        let id = &self.scene.attach(mesh);
        self.primitives.insert(id.unwrap() as usize, Primitive::new(material));
    }

    pub fn build(self) -> Scene {
        Scene {
            scene: self.scene.build(),
            primitives: self.primitives,
            skybox: self.skybox,
            lights: self.lights,
        }
    }
}