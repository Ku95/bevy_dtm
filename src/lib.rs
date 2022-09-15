use anyhow::anyhow;
use bevy::{
    asset::{AssetLoader, BoxedFuture, Error, LoadContext, LoadedAsset},
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use bytemuck::{cast_slice, cast_vec};
use dtm::DTM;

struct DTMAssetLoader;

impl AssetLoader for DTMAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), Error>> {
        Box::pin(async move {
            let (descriptor, pixels) = DTM::decode_alloc(bytes)?;
            let data = Vec::from(cast_slice(&pixels)); // todo: eliminate this copy ?

            load_context.set_default_asset(LoadedAsset::new(Image::new(
                Extent3d {
                    width: descriptor.width as u32,
                    height: descriptor.height as u32,
                    ..default()
                },
                TextureDimension::D2,
                data,
                match descriptor.channel_count {
                    1 => TextureFormat::R16Unorm,
                    2 => TextureFormat::Rg16Unorm,
                    3 => {
                        return Err(anyhow!(
                            "Can not load image: 3 channel images are not supported."
                        ))
                    }
                    4 => TextureFormat::Rgba16Unorm,
                    _ => return Err(anyhow!("Can not load image: invalid channel count.")),
                },
            )));

            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["dtm"]
    }
}

/// Plugin that registers the [`DTMAssetLoader`].
pub struct DTMPlugin;

impl Plugin for DTMPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset_loader(DTMAssetLoader);
    }
}
