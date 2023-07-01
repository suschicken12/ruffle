use crate::{
    bitmap::BitmapHandle,
    pixel_bender::{PixelBenderShaderArgument, PixelBenderShaderHandle},
};
use downcast_rs::{impl_downcast, Downcast};
use gc_arena::Collect;
use std::fmt::Debug;
use swf::Color;

#[derive(Debug, Clone)]
pub enum Filter {
    BevelFilter(swf::BevelFilter),
    BlurFilter(swf::BlurFilter),
    ColorMatrixFilter(swf::ColorMatrixFilter),
    ConvolutionFilter(swf::ConvolutionFilter),
    DisplacementMapFilter(DisplacementMapFilter),
    DropShadowFilter(swf::DropShadowFilter),
    GlowFilter(swf::GlowFilter),
    GradientBevelFilter(swf::GradientFilter),
    GradientGlowFilter(swf::GradientFilter),
    ShaderFilter(ShaderFilter<'static>),
}

#[derive(Debug, Clone)]
pub struct ShaderFilter<'a> {
    pub bottom_extension: i32,
    pub left_extension: i32,
    pub right_extension: i32,
    pub top_extension: i32,
    /// The AVM2 `flash.display.Shader` object that we extracted
    /// the `shader` and `shader_args` fields from. This is used when
    /// we reconstruct a `ShaderFilter` object in the AVM2 `DisplayObject.filters`
    /// (Flash re-uses the same object)
    pub shader_object: Box<dyn ShaderObject>,
    pub shader: PixelBenderShaderHandle,
    pub shader_args: Vec<PixelBenderShaderArgument<'a>>,
}

pub trait ShaderObject: Downcast + Collect + Debug {
    fn clone_box(&self) -> Box<dyn ShaderObject>;
}
impl_downcast!(ShaderObject);

impl Clone for Box<dyn ShaderObject> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl From<&swf::Filter> for Filter {
    fn from(value: &swf::Filter) -> Self {
        match value {
            swf::Filter::DropShadowFilter(filter) => {
                Filter::DropShadowFilter(filter.as_ref().to_owned())
            }
            swf::Filter::BlurFilter(filter) => Filter::BlurFilter(filter.as_ref().to_owned()),
            swf::Filter::GlowFilter(filter) => Filter::GlowFilter(filter.as_ref().to_owned()),
            swf::Filter::BevelFilter(filter) => Filter::BevelFilter(filter.as_ref().to_owned()),
            swf::Filter::GradientGlowFilter(filter) => {
                Filter::GradientGlowFilter(filter.as_ref().to_owned())
            }
            swf::Filter::ConvolutionFilter(filter) => {
                Filter::ConvolutionFilter(filter.as_ref().to_owned())
            }
            swf::Filter::ColorMatrixFilter(filter) => {
                Filter::ColorMatrixFilter(filter.as_ref().to_owned())
            }
            swf::Filter::GradientBevelFilter(filter) => {
                Filter::GradientBevelFilter(filter.as_ref().to_owned())
            }
        }
    }
}

impl Default for Filter {
    fn default() -> Self {
        // A default colormatrix is a filter that essentially does nothing,
        // making it a useful default in situations that we need a dummy filter
        Filter::ColorMatrixFilter(swf::ColorMatrixFilter::default())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum DisplacementMapFilterComponent {
    Alpha,
    Blue,
    Green,
    Red,
}

#[derive(Debug, Clone, Copy)]
pub enum DisplacementMapFilterMode {
    Clamp,
    Color,
    Ignore,
    Wrap,
}

#[derive(Debug, Clone)]
pub struct DisplacementMapFilter {
    pub color: Color,
    pub component_x: u8,
    pub component_y: u8,
    pub map_bitmap: Option<BitmapHandle>,
    pub map_point: (i32, i32),
    pub mode: DisplacementMapFilterMode,
    pub scale_x: f32,
    pub scale_y: f32,
}

impl Default for DisplacementMapFilter {
    fn default() -> Self {
        Self {
            color: Color::from_rgba(0),
            component_x: 0,
            component_y: 0,
            map_bitmap: None,
            map_point: (0, 0),
            mode: DisplacementMapFilterMode::Wrap,
            scale_x: 0.0,
            scale_y: 0.0,
        }
    }
}
