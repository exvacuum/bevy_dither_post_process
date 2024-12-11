use bevy::{
    core_pipeline::fullscreen_vertex_shader::fullscreen_shader_vertex_state,
    prelude::*,
    render::{
        render_resource::{
            binding_types::{sampler, texture_2d},
            BindGroupLayout, BindGroupLayoutEntries, CachedRenderPipelineId, ColorTargetState,
            ColorWrites, FragmentState, MultisampleState, PipelineCache, PrimitiveState,
            RenderPipelineDescriptor, Sampler, SamplerBindingType, SamplerDescriptor, ShaderStages,
            TextureFormat, TextureSampleType,
        },
        renderer::RenderDevice,
    },
};

#[derive(Resource)]
pub struct DitherPostProcessPipeline {
    pub layout: BindGroupLayout,
    pub screen_sampler: Sampler,
    pub threshold_map_sampler: Sampler,
    pub pipeline_id: CachedRenderPipelineId,
}

impl FromWorld for DitherPostProcessPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();

        let layout = render_device.create_bind_group_layout(
            "dither_post_process_bind_group_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::FRAGMENT,
                (
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    sampler(SamplerBindingType::Filtering),
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    sampler(SamplerBindingType::Filtering),
                ),
            ),
        );

        let screen_sampler = render_device.create_sampler(&SamplerDescriptor::default());
        let threshold_map_sampler = render_device.create_sampler(&SamplerDescriptor::default());

        let shader = world.resource::<AssetServer>().load::<Shader>(
            "embedded://bevy_dither_post_process/../assets/shaders/dither_post_process.wgsl",
        );

        let pipeline_id =
            world
                .resource_mut::<PipelineCache>()
                .queue_render_pipeline(RenderPipelineDescriptor {
                    label: Some("dither_post_process_render_pipeline".into()),
                    layout: vec![layout.clone()],
                    push_constant_ranges: vec![],
                    vertex: fullscreen_shader_vertex_state(),
                    primitive: PrimitiveState::default(),
                    depth_stencil: None,
                    multisample: MultisampleState::default(),
                    fragment: Some(FragmentState {
                        shader,
                        shader_defs: vec![],
                        entry_point: "fragment".into(),
                        targets: vec![Some(ColorTargetState {
                            format: TextureFormat::bevy_default(),
                            blend: None,
                            write_mask: ColorWrites::ALL,
                        })],
                    }),
                    zero_initialize_workgroup_memory: false,
                });

        Self {
            layout,
            screen_sampler,
            threshold_map_sampler,
            pipeline_id,
        }
    }
}
