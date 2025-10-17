//! Custom Bevy ECS bundle for shapes.

use bevy::prelude::*;
use lyon_algorithms::path::Builder;
use lyon_tessellation::{self as tess};

use crate::{
    draw::{Fill, Stroke},
    geometry::Geometry,
    plugin::COLOR_MATERIAL_HANDLE,
};

/// `Component` describing a geometric shape.
///
/// It can be constructed using `ShapeBuilder`.
#[derive(Component, Default, Clone)]
#[require(Mesh2d, MeshMaterial2d<ColorMaterial> = color_material_handle(), Transform, Visibility)]
#[non_exhaustive]
pub struct Shape {
    /// Geometry of a shape.
    pub path: tess::path::Path,
    /// Fill data, changes are propagated to the mesh.
    pub fill: Option<Fill>,
    /// Stroke data, changes are propagated to the mesh.
    pub stroke: Option<Stroke>,
}

impl Shape {
    pub(crate) fn new(path: tess::path::Path, fill: Option<Fill>, stroke: Option<Stroke>) -> Self {
        Self { path, fill, stroke }
    }
}

impl Geometry<Builder> for Shape {
    fn add_geometry(&self, b: &mut Builder) {
        b.extend_from_paths(&[self.path.as_slice()]);
    }
}

fn color_material_handle() -> MeshMaterial2d<ColorMaterial> {
    MeshMaterial2d(COLOR_MATERIAL_HANDLE)
}
