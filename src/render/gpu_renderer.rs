use crate::camera::Camera;
use crate::scene::RuntimeScene;
use crate::render::{Frame, RenderContext, Renderer};

pub struct GpuRenderer;

impl GpuRenderer
{
    pub fn new() -> Self
    {
        Self
    }
}

impl Default for GpuRenderer
{
    fn default() -> Self
    {
        Self::new()
    }
}

impl Renderer for GpuRenderer
{
    fn render(
        &mut self,
        _frame: &mut Frame,
        _camera: &Camera,
        _runtime_scene: &RuntimeScene,
        _ctx: &RenderContext,
    ) -> Result<(), String>
    {
        Err("GpuRenderer is not implemented yet. TODO: Add a wgpu backend in src/platform and src/render/gpu.rs.".to_string())
    }
}
