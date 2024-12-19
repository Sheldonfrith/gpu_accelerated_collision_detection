use std::collections::HashSet;

use bevy::{
    prelude::{Entity, Resource, Transform},
    render::{
        render_resource::{BindGroup, BindGroupLayout, Buffer, ComputePipeline},
        renderer::RenderDevice,
    },
};

use super::{
    entity_metadata::CollidableMetadata,
    single_batch::convert_collidables_to_wgsl_types::PerCollidableDataRequiredByGpu,
};

#[derive(Resource)]
pub struct WgslFile(pub String);
// Resources to store reusable GPU state

#[derive(Resource)]
pub struct BindGroupLayoutsResource(pub BindGroupLayout);

#[derive(Resource)]
pub struct PipelineLayoutResource(pub wgpu::PipelineLayout);

#[derive(Resource)]
pub struct CounterStagingBuffer(pub Buffer);

#[derive(Debug, Resource)]
pub struct MaxDetectableCollisionsScale(pub f32);
#[derive(Debug, Resource)]
pub struct WorkgroupSize(pub u32);

#[derive(Clone, Resource)]
pub struct AllCollidablesThisFrame(pub Vec<PerCollidableDataRequiredByGpu>);

#[derive(Resource)]
pub struct MaxBatchSize(pub usize);
