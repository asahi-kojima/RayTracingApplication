use std::any::Any;
use std::time::{Duration, Instant};

use crate::camera::{Camera, CameraSnapshot};
use crate::input::{InputEvent, Key};
use crate::platform::Presenter;
use crate::render::{Frame, RenderContext, Renderer};

const MOVE_SPEED: f64 = 0.1;
const ROT_SPEED_RAD: f64 = 3.0_f64.to_radians();


pub struct App
{
    renderer: Box<dyn Renderer>,
    presenter: Box<dyn Presenter>,
    camera: Camera,
    render_target: Frame,
    fps: u32,
}

impl App
{
    pub fn new(
        renderer: Box<dyn Renderer>,
        presenter: Box<dyn Presenter>,
        camera: Camera,
        fps: u32,
    ) -> Self
    {
        let (width, height) = presenter.size();
        let render_target = Frame::new(width, height);

        Self {
            renderer,
            presenter,
            camera,
            render_target,
            fps,
        }
    }

    pub fn run(&mut self) -> Result<(), String>
    {
        let start = Instant::now();
        let mut frame_index = 0_u64;
        loop
        {
            let mut events = Vec::<InputEvent>::new();
            if self.presenter.handle_events(&mut events)
            {
                return Ok(());
            }

            for event in events
            {
                match event
                {
                    InputEvent::KeyDown(key)=>
                    {
                        match key
                        {
                            // WASD: 平行移動
                            Key::W => self.camera.move_forward(MOVE_SPEED),
                            Key::S => self.camera.move_forward(-MOVE_SPEED),
                            Key::A => self.camera.move_right(-MOVE_SPEED),
                            Key::D => self.camera.move_right(MOVE_SPEED),

                            Key::P => 
                            {
                                println!("Save scene data to file");

                                let parent = std::path::Path::new("storage");
                                let filename = parent.join("scene.json");

                                std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
                                self.save_scene_data(filename.to_str().ok_or("Invalid filename")?)?;
                            },

                            // 矢印: 視点回転
                            Key::LEFT => self.camera.yaw(ROT_SPEED_RAD),
                            Key::RIGHT => self.camera.yaw(-ROT_SPEED_RAD),
                            Key::UP => self.camera.pitch(ROT_SPEED_RAD),
                            Key::DOWN => self.camera.pitch(-ROT_SPEED_RAD),

                            Key::ESCAPE => return Ok(()),
                            _ => {}
                        }
                    },
                    InputEvent::MouseMove { x, y, dx, dy } =>
                    {
                        // 例: 右ドラッグ中だけ視点回転に使うなど
                        println!("mouse move: ({x},{y}) rel=({dx},{dy})");
                    },
                    InputEvent::MouseDown { button, x, y } =>
                    {
                        // クリック検知
                        println!("mouse down: {:?} at ({x},{y})", button);
                    },
                    InputEvent::MouseUp { button, x, y } =>
                    {
                        // リリース検知
                        println!("mouse up: {:?} at ({x},{y})", button);
                    },
                    InputEvent::MouseWheel { x, y } =>
                    {
                        // 例: y>0で前進、y<0で後退
                        println!("mouse wheel: ({x},{y})");
                    },

                    InputEvent::TextInput(text) =>
                    {
                        println!("TextInput: {}", text);
                    },
                    _ => println!("Unhandled event: {:?}", event.type_id()),
                }
            }

            let render_context = RenderContext {
                frame_index,
                elapsed_seconds: start.elapsed().as_secs_f32(),
            };

            self.renderer.render(&mut self.render_target, &self.camera, &render_context)?;
            self.presenter.present(&self.render_target)?;

            frame_index += 1;
            std::thread::sleep(Duration::from_millis(16));
        }
    }


    fn save_scene_data(&self, filename: &str) -> Result<(), String>
    {
        let snap_shot = CameraSnapshot::from_camera(&self.camera);
        println!("Saving scene data to {}: {:?}", filename, snap_shot);
        std::fs::write(filename, serde_json::to_string(&snap_shot).unwrap()).map_err(|e| e.to_string())
    }

}
