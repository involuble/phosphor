use cgmath::*;
use embree;
use embree::{BuildQuality, SceneFlags, Hit, GeomID};
use vec_map::VecMap;

use crate::math::*;
use crate::colour::*;
use crate::material_type::*;
use crate::materials::*;
use crate::geometry::*;

pub struct Scene {
    scene: embree::Scene,
    primitives: VecMap<Primitive>,
    skybox: Colour,
    pub lights: Vec<(GeomID, Box<SampleableEmitter>)>,
}

struct Primitive {
    pub emitter: EmissiveGeometry,
    pub material: MaterialType,
    // pub tex_scale: Vector2<f32>,
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

// TODO: private
#[derive(Clone)]
pub enum EmissiveGeometry {
    NotEmissive,
    Sphere(Sphere),
    Quad(Quad),
}

pub struct ShadingParameters {
    pub bsdf: Box<Bsdf>,
    pub basis: TangentFrame,
}

impl Scene {
    pub fn intersect(&self, ray: &Ray) -> Hit {
        self.scene.intersect((*ray).into())
    }

    pub fn skybox_emission(&self, _dir: Vector3<f32>) -> Colour {
        self.skybox
    }

    pub fn emission_at(&self, ray: &Ray, hit: &Hit) -> LightSample {
        let p = ray.point_at_dist(hit.t);
        let e = &self.primitives[hit.geom_id.unwrap() as usize].emitter;
        match e {
            EmissiveGeometry::NotEmissive => LightSample {
                dir: Vector3::zero(),
                distance: 0.0,
                radiance: Colour::zero(),
                pdf: PdfW(1.0),
            },
            EmissiveGeometry::Sphere(s) => s.eval_emission_at(ray.origin, p),
            EmissiveGeometry::Quad(q) => q.eval_emission_at(ray.origin, p),
        }
    }
    
    pub fn shading_parameters_at(&self, hit: &Hit) -> ShadingParameters {
        ShadingParameters {
            bsdf: self.primitives[hit.geom_id.unwrap() as usize].material.compute_bsdf(hit),
            basis: TangentFrame::from_normal(hit.Ng),
        }
    }
}

pub struct SceneBuilder {
    pub device: embree::Device,
    scene: embree::SceneBuilder,
    primitives: VecMap<Primitive>,
    skybox: Colour,
    lights: Vec<(GeomID, Box<SampleableEmitter>)>,
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

        let user = embree::UserGeometry::new(&self.device, vec![sphere.clone()]);
        let id = self.scene.attach(user.build());
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

        let index = vec![embree::Triangle::new(0, 1, 2), embree::Triangle::new(0, 2, 3)];
        let mesh = embree::TriangleMesh::new(&self.device, index, Vec::from(quad.points().as_ref()));
        let id = self.scene.attach(mesh.build());
        self.primitives.insert(id.unwrap() as usize, prim);
        if quad.is_emissive() {
            self.lights.push((id, Box::new(quad)));
        }
    }

    pub fn add_mesh(&mut self, mesh: embree::TriangleMesh, material: MaterialType) {
        // TODO: Probably shouldn't expose embree like this
        let id = &self.scene.attach(mesh.build());
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