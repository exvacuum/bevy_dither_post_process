use bevy::{
    prelude::*,
    render::{
        extract_component::ExtractComponent,
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
    },
};

/// Component which, when inserted into an entity with a camera, enables the dither post-processing
/// effect.
#[derive(Component, ExtractComponent, Clone)]
pub struct DitherPostProcessSettings(Handle<Image>);

impl DitherPostProcessSettings {
    /// Constructs a new instance ofthis component, enabling the dither effect using a bayer
    /// matrix of the given level. A given level *n* will generate a square bayer matrix with a size of *(n+1)^2*.
    pub fn new(level: u32, asset_server: &AssetServer) -> Self {
        let power = level + 1;
        let map_size: u32 = 1 << power;
        let mut buffer = Vec::<u8>::new();

        for row in 0..map_size {
            for col in 0..map_size {
                let a = row ^ col;
                // Interleave bits of `a` with bits of y coordinate in reverse order
                let mut result: u64 = 0;
                let mut bit = 0;
                let mut mask = power as i32 - 1;
                loop {
                    if bit >= 2 * power {
                        break;
                    }
                    result |= (((col >> mask) & 1) << bit) as u64;
                    bit += 1;
                    result |= (((a >> mask) & 1) << bit) as u64;
                    bit += 1;
                    mask -= 1;
                }
                let value = ((result as f32 / map_size.pow(2) as f32) * 255.0) as u8;
                buffer.push(value);
            }
        }

        let mut image = Image::new(
            Extent3d {
                width: map_size,
                height: map_size,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            buffer,
            TextureFormat::R8Unorm,
            RenderAssetUsages::RENDER_WORLD,
        );
        image.texture_descriptor.usage = TextureUsages::COPY_DST
            | TextureUsages::STORAGE_BINDING
            | TextureUsages::TEXTURE_BINDING;

        let handle = asset_server.add(image);

        Self(handle)
    }
    /// Gets the handle of the texture representing this component's Bayer matrix
    pub fn handle(&self) -> Handle<Image> {    
        self.0.clone()
    }
}
