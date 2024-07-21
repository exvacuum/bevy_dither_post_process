#![warn(missing_docs)]

//! A plugin for the Bevy game engine which provides a black and white dither post-process effect
//! using Bayer ordered dithering.

use bevy::{
    asset::embedded_asset,
    core_pipeline::core_3d::graph::{Core3d, Node3d},
    prelude::*,
    render::{
        extract_component::ExtractComponentPlugin,
        render_graph::{RenderGraphApp, ViewNodeRunner},
        RenderApp,
    },
};

use crate::components::DitherPostProcessSettings;

pub use nodes::DitherRenderLabel;

/// Plugin which provides dither post-processing functionality
pub struct DitherPostProcessPlugin;

/// Components used by this plugin
pub mod components;

mod nodes;
mod resources;

impl Plugin for DitherPostProcessPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "../assets/shaders/dither_post_process.wgsl");

        app.add_plugins((ExtractComponentPlugin::<DitherPostProcessSettings>::default(),));

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .add_render_graph_node::<ViewNodeRunner<nodes::DitherRenderNode>>(
                Core3d,
                nodes::DitherRenderLabel,
            )
            .add_render_graph_edges(
                Core3d,
                (
                    Node3d::Tonemapping,
                    nodes::DitherRenderLabel,
                    Node3d::EndMainPassPostProcessing,
                ),
            );
    }

    fn finish(&self, app: &mut App) {
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app.init_resource::<resources::DitherPostProcessPipeline>();
    }
}
