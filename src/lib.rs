use bevy::{prelude::*, render::{RenderApp, render_graph::{RenderGraphApp, ViewNodeRunner}, extract_component::ExtractComponentPlugin}, asset::embedded_asset, core_pipeline::core_3d::graph::{Core3d, Node3d}};

use crate::components::DitherPostProcessSettings;

pub use nodes::DitherRenderLabel;

pub struct DitherPostProcessPlugin;

pub mod components;
mod resources;
mod nodes;

impl Plugin for DitherPostProcessPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "../assets/shaders/dither_post_process.wgsl");

        app.add_plugins((
            ExtractComponentPlugin::<DitherPostProcessSettings>::default(),
        ));

        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app.add_render_graph_node::<ViewNodeRunner<nodes::DitherRenderNode>>(
            Core3d,
            nodes::DitherRenderLabel,
        ).add_render_graph_edges(
            Core3d, 
            (
                Node3d::Tonemapping,
                nodes::DitherRenderLabel,
                Node3d::EndMainPassPostProcessing,
            ),
        );
    }
    
    fn finish(&self, app: &mut App) {
        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app.init_resource::<resources::DitherPostProcessPipeline>();
    }
}
