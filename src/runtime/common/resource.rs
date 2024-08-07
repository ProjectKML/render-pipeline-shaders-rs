use std::mem;

use bitflags::bitflags;

use crate::{sys, utils::assert_size_and_align, Format, INDEX_NONE_U32};

pub const RESOURCE_ID_INVALID: u32 = INDEX_NONE_U32;

pub type ResourceId = u32;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ResourceType(u32);

impl ResourceType {
    pub const UNKNOWN: Self = Self(sys::RpsResourceType_RPS_RESOURCE_TYPE_UNKNOWN as _);
    pub const BUFFER: Self = Self(sys::RpsResourceType_RPS_RESOURCE_TYPE_BUFFER as _);
    pub const IMAGE_1D: Self = Self(sys::RpsResourceType_RPS_RESOURCE_TYPE_IMAGE_1D as _);
    pub const IMAGE_2D: Self = Self(sys::RpsResourceType_RPS_RESOURCE_TYPE_IMAGE_2D as _);
    pub const IMAGE_3D: Self = Self(sys::RpsResourceType_RPS_RESOURCE_TYPE_IMAGE_3D as _);
    pub const COUNT: Self = Self(sys::RpsResourceType_RPS_RESOURCE_TYPE_COUNT as _);
}

impl ResourceType {
    #[inline]
    pub fn from_raw(raw: u32) -> Self {
        Self(raw)
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct ResourceFlags: u32 {
        const NONE = sys::RpsResourceFlagBits_RPS_RESOURCE_FLAG_NONE as _;
        const CUBEMAP_COMPATIBLE = sys::RpsResourceFlagBits_RPS_RESOURCE_FLAG_CUBEMAP_COMPATIBLE_BIT as _;
        const ROWMAJOR_IMAGE = sys::RpsResourceFlagBits_RPS_RESOURCE_FLAG_ROWMAJOR_IMAGE_BIT as _;
        const PREFER_GPU_LOCAL_CPU_VISIBLE = sys::RpsResourceFlagBits_RPS_RESOURCE_FLAG_PREFER_GPU_LOCAL_CPU_VISIBLE_BIT as _;
        const PREFER_DEDICATED_ALLOCATION =sys::RpsResourceFlagBits_RPS_RESOURCE_FLAG_PREFER_DEDICATED_ALLOCATION_BIT as _;
        const PERSISTENT = sys::RpsResourceFlagBits_RPS_RESOURCE_FLAG_PERSISTENT_BIT as _;
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct ImageAspectUsageFlags: u32 {
        const UNKNOWN = sys::RpsImageAspectUsageFlagBits_RPS_IMAGE_ASPECT_UNKNOWN as _;
        const COLOR = sys::RpsImageAspectUsageFlagBits_RPS_IMAGE_ASPECT_COLOR as _;
        const DEPTH = sys::RpsImageAspectUsageFlagBits_RPS_IMAGE_ASPECT_DEPTH as _;
        const STENCIL = sys::RpsImageAspectUsageFlagBits_RPS_IMAGE_ASPECT_STENCIL as _;
        const METADATA = sys::RpsImageAspectUsageFlagBits_RPS_IMAGE_ASPECT_METADATA as _;
        const DEFAULT = sys::RpsImageAspectUsageFlagBits_RPS_IMAGE_ASPECT_DEFAULT as _;
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union ClearColorValue {
    pub float32: [f32; 4],
    pub int32: [i32; 4],
    pub uint32: [u32; 4]
}

impl Default for ClearColorValue {
    #[inline]
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

assert_size_and_align!(ClearColorValue, sys::RpsClearColorValue);

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct ClearFlags: u32 {
        const NONE = sys::RpsClearFlags_RPS_CLEAR_FLAG_NONE as _;
        const COLOR = sys::RpsClearFlags_RPS_CLEAR_FLAG_COLOR as _;
        const DEPTH = sys::RpsClearFlags_RPS_CLEAR_FLAG_DEPTH as _;
        const STENCIL = sys::RpsClearFlags_RPS_CLEAR_FLAG_STENCIL as _;
        const UAVFLOAT = sys::RpsClearFlags_RPS_CLEAR_FLAG_UAVFLOAT as _;
        const UAVUINT = sys::RpsClearFlags_RPS_CLEAR_FLAG_UAVUINT as _;
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct ClearDepthStencilValue {
    pub depth: f32,
    pub stencil: u32
}

assert_size_and_align!(ClearDepthStencilValue, sys::RpsClearDepthStencilValue);

#[repr(C)]
#[derive(Clone, Copy)]
pub union ClearValue {
    pub color: ClearColorValue,
    pub depth_stencil: ClearDepthStencilValue
}

impl Default for ClearValue {
    #[inline]
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

assert_size_and_align!(ClearValue, sys::RpsClearValue);

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ClearInfo {
    pub format: Format,
    pub value: ClearValue
}

assert_size_and_align!(ClearInfo, sys::RpsClearInfo);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct ResourceImageDesc {
    pub width: u32,
    pub height: u32,
    pub depth_or_array_layers: u32,
    pub mip_levels: u32,
    pub format: Format,
    pub sample_count: u32
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct ResourceBufferDesc {
    pub size_in_bytes_lo: u32,
    pub size_in_bytes_hi: u32
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union ResourceBufferImageDesc {
    pub image: ResourceImageDesc,
    pub buffer: ResourceBufferDesc
}

impl Default for ResourceBufferImageDesc {
    #[inline]
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ResourceDesc {
    pub type_: ResourceType,
    pub temporal_layers: u32,
    pub flags: ResourceFlags,
    pub buffer_image: ResourceBufferImageDesc
}

assert_size_and_align!(ResourceDesc, sys::RpsResourceDesc);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct SubresourceRange {
    pub base_mip_level: u16,
    pub mip_levels: u16,
    pub base_array_layer: u32,
    pub array_layers: u32
}

assert_size_and_align!(SubresourceRange, sys::RpsSubresourceRange);

pub const RESOURCE_MAX_TEMPORAL_LAYERS: usize = 256;
pub const MAX_SIMULTANEOUS_RENDER_TARGET_COUNT: usize = 8;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct CmdRenderTargetInfo {
    pub num_render_targets: u32,
    pub num_samples: u32,
    pub depth_stencil_format: Format,
    pub render_target_formats: [Format; MAX_SIMULTANEOUS_RENDER_TARGET_COUNT]
}

assert_size_and_align!(CmdRenderTargetInfo, sys::RpsCmdRenderTargetInfo);
