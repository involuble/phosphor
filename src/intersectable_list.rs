use primitives::*;

#[derive(Debug)]
pub struct IntersectableList<T> where T: Intersectable {
    pub geom: Vec<T>,
}

impl<T> IntersectableList<T> where T: Intersectable {
    pub fn build(geom: Vec<T>) -> Self {
        IntersectableList {
            geom: geom,
        }
    }
}

impl<T> Intersectable for IntersectableList<T> where T: Intersectable {
    fn intersect(&self, ray: &Ray) -> Intersection {
        let mut hit = Intersection::miss();
        for i in &self.geom {
            let new_hit = i.intersect(&ray);
            Intersection::replace_closest(&mut hit, &new_hit, EPSILON);
        }
        hit
    }
}
