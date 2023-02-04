use bevy::{
    asset::load_internal_asset,
    prelude::{AddAsset, App, Assets, Handle, HandleUntyped, Plugin, Shader},
    reflect::{prelude::*, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin},
};

pub const SHAPE_MATERIAL_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 3191283017262752456);

pub(crate) struct ShapeMaterialPlugin;

impl Plugin for ShapeMaterialPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            SHAPE_MATERIAL_SHADER_HANDLE,
            "shape_material.wgsl",
            Shader::from_wgsl
        );

        app.add_plugin(Material2dPlugin::<ShapeMaterial>::default())
            .register_asset_reflect::<ShapeMaterial>();

        app.world
            .resource_mut::<Assets<ShapeMaterial>>()
            .set_untracked(Handle::<ShapeMaterial>::default(), ShapeMaterial::default());
    }
}

/// The Material trait is very configurable, but comes with sensible defaults
/// for all methods. You only need to implement functions for features that need
/// non-default behavior. See the Material api docs for details!
impl Material2d for ShapeMaterial {
    fn fragment_shader() -> ShaderRef {
        SHAPE_MATERIAL_SHADER_HANDLE.typed().into()
    }
}

// This is the struct that will be passed to your shader
#[derive(Default, AsBindGroup, Reflect, FromReflect, Debug, Clone, TypeUuid)]
#[reflect(Default, Debug)]
#[uuid = "ab2e068e-0cca-4941-a114-524af2c431bb"]
pub struct ShapeMaterial {}
