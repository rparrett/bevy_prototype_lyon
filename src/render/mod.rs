use bevy::{
    asset::load_internal_asset,
    prelude::{AddAsset, App, Assets, Handle, HandleUntyped, Image, Plugin, Shader},
    reflect::{prelude::*, TypeUuid},
    render::{
        render_asset::RenderAssets,
        render_resource::{AsBindGroup, AsBindGroupShaderType, ShaderRef, ShaderType},
    },
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
#[uniform(0, ShapeMaterialUniform)]
pub struct ShapeMaterial {
    pub flags: u32,
    #[texture(1)]
    #[sampler(2)]
    pub texture: Option<Handle<Image>>,
}

// NOTE: These must match the bit flags in shape_material.wgsl!
bitflags::bitflags! {
    #[repr(transparent)]
    pub struct ShapeMaterialFlags: u32 {
        const TEXTURE           = (1 << 0);
        const NONE              = 0;
        const UNINITIALIZED     = 0xFFFF;
    }
}

/// The GPU representation of the uniform data of a [`ShapeMaterial`].
#[derive(Clone, Default, ShaderType)]
pub struct ShapeMaterialUniform {
    pub flags: u32,
}

impl AsBindGroupShaderType<ShapeMaterialUniform> for ShapeMaterial {
    fn as_bind_group_shader_type(&self, _images: &RenderAssets<Image>) -> ShapeMaterialUniform {
        let mut flags = ShapeMaterialFlags::NONE;
        if self.texture.is_some() {
            flags |= ShapeMaterialFlags::TEXTURE;
        }

        ShapeMaterialUniform {
            flags: flags.bits(),
        }
    }
}
