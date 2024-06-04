use bevy::{
    ecs::query::QueryItem,
    prelude::*,
    render::{
        render_asset::RenderAssets,
        render_graph::{NodeRunError, RenderGraphContext, RenderLabel, ViewNode},
        render_resource::{
            BindGroupEntries, Operations, PipelineCache, RenderPassColorAttachment,
            RenderPassDescriptor,
        },
        renderer::RenderContext,
        view::ViewTarget,
    },
};

use super::components;
use super::resources;

/// Label for dither post-process effect render node.
#[derive(RenderLabel, Clone, Eq, PartialEq, Hash, Debug)]
pub struct DitherRenderLabel;

#[derive(Default)]
pub struct DitherRenderNode;

impl ViewNode for DitherRenderNode {
    type ViewQuery = (
        &'static ViewTarget,
        &'static components::DitherPostProcessSettings,
    );

    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        (view_target, dither_post_process_settings): QueryItem<Self::ViewQuery>,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let render_pipeline = world.resource::<resources::DitherPostProcessPipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();
        let Some(pipeline) = pipeline_cache.get_render_pipeline(render_pipeline.pipeline_id) else {
            warn!("Failed to get render pipeline from cache, skipping...");
            return Ok(());
        };

        let post_process = view_target.post_process_write();

        let Some(threshold_map) = world
            .resource::<RenderAssets<Image>>()
            .get(dither_post_process_settings.handle())
        else {
            warn!("Failed to get threshold map, skipping...");
            return Ok(());
        };

        let bind_group = render_context.render_device().create_bind_group(
            "dither_post_process_bind_group",
            &render_pipeline.layout,
            &BindGroupEntries::sequential((
                post_process.source,
                &render_pipeline.screen_sampler,
                &threshold_map.texture_view,
                &render_pipeline.threshold_map_sampler,
            )),
        );

        let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
            label: Some("dither_post_process_render_pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: post_process.destination,
                ops: Operations::default(),
                resolve_target: None,
            })],
            timestamp_writes: None,
            depth_stencil_attachment: None,
            occlusion_query_set: None,
        });

        render_pass.set_render_pipeline(pipeline);
        render_pass.set_bind_group(0, &bind_group, &[]);
        render_pass.draw(0..3, 0..1);
        Ok(())
    }
}
