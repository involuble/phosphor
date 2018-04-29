use cgmath::*;
use embree;
use embree::{BuildQuality, SceneFlags, Hit, GeomID};
use vec_map::VecMap;

use math::*;
use colour::*;
use material::*;
use geometry::*;

pub struct Scene {
    scene: embree::Scene,
    primitives: VecMap<Primitive>,
    pub skybox: Colour,
    // lights: Vec<(GeomID, EmissiveGeometry)>,
}

struct Primitive {
    pub emitter: EmissiveGeometry,
    pub material: Material,
    // pub tex_scale: Vector2<f32>,
    // pub normal_map: Texture,
}

enum EmissiveGeometry {
    NotEmissive,
    Sphere(Sphere),
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Copy)]
pub struct ShadingParameters<'a> {
    pub material: &'a Material,
    pub Ns: Vector3<f32>,
    pub tangent: Vector3<f32>,
}

impl Scene {
    pub fn intersect(&self, ray: &Ray) -> Hit {
        self.scene.intersect((*ray).into())
    }

    pub fn eval_skybox(&self, _: Vector3<f32>) -> Colour {
        self.skybox
    }

    pub fn emission_at(&self, ray: &Ray, hit: &Hit) -> LightSample {
        let p = ray.point_at_dist(hit.t);
        let e = &self.primitives[hit.geom_id.unwrap() as usize].emitter;
        match *e {
            EmissiveGeometry::NotEmissive => LightSample {
                direction: Vector3::zero(),
                radiance: Colour::zero(),
                pdf: PdfW(0.0),
            },
            EmissiveGeometry::Sphere(ref s) => s.eval_emission_at(ray.origin, p)
        }
    }
    
    pub fn get_material(&self, id: GeomID) -> &Material {
        &self.primitives[id.unwrap() as usize].material
    }
}

pub struct SceneBuilder {
    device: embree::Device,
    scene: embree::SceneBuilder,
    primitives: VecMap<Primitive>,
    skybox: Colour,
}

impl SceneBuilder {
    pub fn new(device: &embree::Device) -> Self {
        let mut s = embree::SceneBuilder::new(device);
        s.set_build_quality(BuildQuality::Medium);
        s.set_flags(SceneFlags::ROBUST | SceneFlags::COMPACT);

        SceneBuilder {
            device: device.clone(),
            skybox: Colour::zero(),
            scene: s,
            primitives: VecMap::new(),
        }
    }

    pub fn add_emissive_sphere(&mut self, sphere: Sphere) {
        debug_assert!(!sphere.emission.is_zero(), "Sphere is not emissive");

        let user = embree::UserGeometry::new(&self.device, vec![sphere.clone()]);
        let id = &self.scene.attach(user.build());
        let prim = Primitive {
            emitter: EmissiveGeometry::Sphere(sphere),
            material: Material::Emitter,
        };
        self.primitives.insert(id.unwrap() as usize, prim);
    }

    pub fn add_sphere(&mut self, sphere: Sphere, mat: Material) {
        let user = embree::UserGeometry::new(&self.device, vec![sphere]);
        let id = &self.scene.attach(user.build());
        let prim = Primitive {
            emitter: EmissiveGeometry::NotEmissive,
            material: mat,
        };
        self.primitives.insert(id.unwrap() as usize, prim);
    }

    pub fn build(self) -> Scene {
        Scene {
            scene: self.scene.build(),
            primitives: self.primitives,
            skybox: self.skybox,
        }
    }
}