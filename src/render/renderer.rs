use crate::camera::Camera;
use crate::scene::RuntimeScene;
use crate::render::Frame;


// 1フレームごとの描画に必要な共通情報を Renderer に渡すためのデータ構造体
#[derive(Debug, Clone, Copy)]
pub struct RenderContext
{
    pub frame_index: u64,
    pub elapsed_seconds: f32,
}

pub trait Renderer
{
    fn render(
        &mut self,
        frame: &mut Frame,
        camera: &Camera,
        runtime_scene: &RuntimeScene,
        ctx: &RenderContext,
    ) -> Result<(), String>;
}