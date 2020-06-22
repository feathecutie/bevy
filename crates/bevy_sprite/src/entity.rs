use crate::{
    render::SPRITE_PIPELINE_HANDLE, sprite::Sprite, ColorMaterial, TextureAtlas,
    TextureAtlasSprite, QUAD_HANDLE, SPRITE_SHEET_PIPELINE_HANDLE,
};
use bevy_app::EntityArchetype;
use bevy_asset::Handle;
use bevy_render::{
    draw::Draw,
    mesh::Mesh,
    pipeline::{DynamicBinding, PipelineSpecialization, RenderPipeline, RenderPipelines},
};
use bevy_transform::prelude::{Rotation, Scale, Transform, Translation};

#[derive(EntityArchetype)]
pub struct SpriteEntity {
    pub sprite: Sprite,
    pub mesh: Handle<Mesh>, // TODO: maybe abstract this out
    pub material: Handle<ColorMaterial>,
    pub draw: Draw,
    pub render_pipelines: RenderPipelines,
    pub transform: Transform,
    pub translation: Translation,
    pub rotation: Rotation,
    pub scale: Scale,
}

impl Default for SpriteEntity {
    fn default() -> Self {
        Self {
            mesh: QUAD_HANDLE,
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::specialized(
                SPRITE_PIPELINE_HANDLE,
                PipelineSpecialization {
                    dynamic_bindings: vec![
                        // Transform
                        DynamicBinding {
                            bind_group: 2,
                            binding: 0,
                        },
                        // Sprite
                        DynamicBinding {
                            bind_group: 2,
                            binding: 1,
                        },
                    ],
                    ..Default::default()
                },
            )]),
            sprite: Default::default(),
            material: Default::default(),
            draw: Default::default(),
            transform: Default::default(),
            translation: Default::default(),
            rotation: Default::default(),
            scale: Default::default(),
        }
    }
}

#[derive(EntityArchetype)]
pub struct SpriteSheetEntity {
    pub sprite: TextureAtlasSprite,
    pub texture_atlas: Handle<TextureAtlas>,
    pub draw: Draw,
    pub render_pipelines: RenderPipelines,
    pub mesh: Handle<Mesh>, // TODO: maybe abstract this out
    pub transform: Transform,
    pub translation: Translation,
    pub rotation: Rotation,
    pub scale: Scale,
}

impl Default for SpriteSheetEntity {
    fn default() -> Self {
        Self {
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::specialized(
                SPRITE_SHEET_PIPELINE_HANDLE,
                PipelineSpecialization {
                    dynamic_bindings: vec![
                        // Transform
                        DynamicBinding {
                            bind_group: 2,
                            binding: 0,
                        },
                        // TextureAtlasSprite
                        DynamicBinding {
                            bind_group: 2,
                            binding: 1,
                        },
                    ],
                    ..Default::default()
                },
            )]),
            mesh: QUAD_HANDLE,
            sprite: Default::default(),
            texture_atlas: Default::default(),
            draw: Default::default(),
            transform: Default::default(),
            translation: Default::default(),
            rotation: Default::default(),
            scale: Default::default(),
        }
    }
}