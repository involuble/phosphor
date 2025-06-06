use glam::*;

use sys::*;

use crate::device::*;
use crate::common::*;
use crate::geometry::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IndexedTriangle {
    pub v0: u32,
    pub v1: u32,
    pub v2: u32,
}

impl IndexedTriangle {
    const POLYGON_TYPE: GeometryType = GeometryType::Triangle;

    pub fn new(v0: u32, v1: u32, v2: u32) -> Self {
        IndexedTriangle {
            v0,
            v1,
            v2,
        }
    }
}

/// A quad is defined as a pair of triangles (v0, v1, v3) & (v2, v3, v1).
/// All of the vertices should be co-planar
/// Triangles and quads can be mixed by using a quad with v2 == v3
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IndexedQuad {
    pub v0: u32,
    pub v1: u32,
    pub v2: u32,
    pub v3: u32,
}

impl IndexedQuad {
    const POLYGON_TYPE: GeometryType = GeometryType::Quad;

    pub fn new(v0: u32, v1: u32, v2: u32, v3: u32) -> Self {
        IndexedQuad {
            v0,
            v1,
            v2,
            v3,
        }
    }
}

impl TypeFormat for IndexedTriangle {
    const FORMAT: Format = Format::u32x3;
}

impl TypeFormat for IndexedQuad {
    const FORMAT: Format = Format::u32x4;
}

// Internal use constants
const NORMALS_SLOT: u32 = 0;
const UV_SLOT: u32 = 1;

macro_rules! polygon_geometry_def {
    ($geometryname:ident, $polygon:ty, $geometry_constructor:ident) => (
pub struct $geometryname {
    pub(crate) handle: GeometryHandle,
    pub indices: Vec<$polygon>,
    pub vertices: Vec<Vec3>,
    pub normals: Option<Vec<Vec3>>,
    pub tex_coords: Option<Vec<Vec2>>,
}

impl $geometryname {
    pub fn new(device: &Device, index_buffer: Vec<$polygon>, vertex_buffer: Vec<Vec3>) -> Self {
        let handle = GeometryHandle::new(device, <$polygon>::POLYGON_TYPE);
        $geometryname {
            handle: handle,
            indices: index_buffer,
            vertices: vertex_buffer,
            normals: None,
            tex_coords: None,
        }
    }

    pub fn set_normal_buffer(&mut self, buf: Vec<Vec3>) {
        self.normals = Some(buf);
    }

    pub fn set_texcoord_buffer(&mut self, buf: Vec<Vec2>) {
        self.tex_coords = Some(buf);
    }

    pub fn transform_mesh(&mut self, transform: Mat4) {
        for v in self.vertices.iter_mut() {
            *v = transform.transform_point3(*v);
        }
        if let Some(ref mut normal_buf) = self.normals {
            let normal_transform = transform.inverse().transpose();
            for n in normal_buf.iter_mut() {
                *n = normal_transform.transform_vector3(*n);
            }
        }
    }
}

impl Geometry for $geometryname {
    fn handle(&self) -> &GeometryHandle {
        &self.handle
    }

    fn handle_mut(&mut self) -> &mut GeometryHandle {
        &mut self.handle
    }

    fn bind_buffers(&mut self) {
        let mut attrib_count = 0;
        if self.normals.is_some() { attrib_count = NORMALS_SLOT + 1; }
        if self.tex_coords.is_some() { attrib_count = UV_SLOT + 1; }

        self.indices.reserve(1);
        self.vertices.reserve(1);
        
        unsafe {
            self.handle.bind_shared_geometry_buffer(&mut self.indices, BufferType::Index, <$polygon>::FORMAT, 0, 0);
            self.handle.bind_shared_geometry_buffer(&mut self.vertices, BufferType::Vertex, Format::f32x3, 0, 0);

            rtcSetGeometryVertexAttributeCount(self.handle.ptr, attrib_count);

            if let Some(ref mut data) = self.normals {
                data.reserve(1);
                self.handle.bind_shared_geometry_buffer(data, BufferType::VertexAttribute, Format::f32x3, NORMALS_SLOT, 0);
            }
            if let Some(ref mut data) = self.tex_coords {
                data.reserve(1);
                self.handle.bind_shared_geometry_buffer(data, BufferType::VertexAttribute, Format::f32x2, UV_SLOT, 0);
            }
        }
    }
}
)}

polygon_geometry_def!(TriangleMesh, IndexedTriangle, Triangles);
polygon_geometry_def!(QuadMesh, IndexedQuad, Quads);
