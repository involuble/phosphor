use cgmath::*;

use geometry::*;

use bbox::*;

trait BoundablePrimitive: Boundable + Intersectable {}
impl<T: ?Sized> BoundablePrimitive for T where T: Boundable + Intersectable {}

pub struct BVHNode<T> where T: BoundablePrimitive {
    pub bbox: BBox,
}
