use crate::internal_prelude::*;
use std::any::Any;
use std::time::{Duration, Instant};

use crate::camera::{Camera, CameraSnapshot};
use crate::input::{InputEvent, Key};
use crate::scene::{RuntimeScene, Scene};
use crate::platform::Presenter;
use crate::render::{Frame, RenderContext, Renderer, CpuRenderer, GpuRenderer};

const MOVE_SPEED: f64 = 0.1;
const ROT_SPEED_RAD: f64 = 3.0_f64.to_radians();


pub struct App
{
    renderer: Box<dyn Renderer>,
    presenter: Box<dyn Presenter>,
    camera: Camera,
    scene: Scene,
    runtime_scene: RuntimeScene,
    scene_dirty: bool,
    render_target: Frame,
    fps: u32,
}

impl App
{
    pub fn new(
        presenter: Box<dyn Presenter>,
        camera: Camera,
        fps: u32,
    ) -> Self
    {
        let backend = std::env::var("RAY_BACKEND").unwrap_or_else(|_| "cpu".to_string());
        let renderer: Box<dyn Renderer> = match backend.as_str()
        {
            "gpu" => Box::new(GpuRenderer::new()),
            _ => Box::new(CpuRenderer::new()),
        };


        let (width, height) = presenter.size();
        let render_target = Frame::new(width, height);
        let scene = Scene::new();
        let runtime_scene = scene
            .compile_to_runtime_scene()
            .expect("empty scene compile should always succeed");

        Self {
            renderer,
            presenter,
            camera,
            scene,
            runtime_scene,
            scene_dirty: false,
            render_target,
            fps,
        }
    }

    pub fn run(&mut self) -> Result<(), String>
    {
        // 開始時刻
        let start = Instant::now();

        // このフレームの終了時刻
        let mut frame_finish_time = Instant::now();
 
        // 前フレームからの経過時間
        let mut delta_time = Duration::from_secs(0);

        // このフレームの開始時刻
        let mut frame_start_time = Instant::now();

        // 開始時点からの経過フレーム
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
                            
                            // 矢印: 視点回転
                            Key::LEFT  => self.camera.yaw(ROT_SPEED_RAD),
                            Key::RIGHT => self.camera.yaw(-ROT_SPEED_RAD),
                            Key::UP    => self.camera.pitch(ROT_SPEED_RAD),
                            Key::DOWN  => self.camera.pitch(-ROT_SPEED_RAD),

                            // シーンの保存
                            Key::P => 
                            {
                                println!("Save scene data to file");

                                let parent = std::path::Path::new("storage");
                                let filename = parent.join("scene.json");

                                std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
                                self.save_scene_data(filename.to_str().ok_or("Invalid filename")?)?;
                            },

                            Key::ESCAPE => return Ok(()),

                            Key::H => 
                            {
                                use crate::scene::{Object, PrimitiveId, MaterialId};
                                let mut object = Object::new(&("Red Sphere".to_string() + &frame_index.to_string()), PrimitiveId(0), MaterialId(0));
                                let transform = Transform::identity();
                                let transform = transform.with_position(Point::random_in_cube(10.0));
                                object.set_transform(transform);
                                self.add_object(object);
                            },

                            _ => {}
                        }
                    },
                    InputEvent::MouseMove { x, y, dx, dy } =>
                    {
                        // 例: 右ドラッグ中だけ視点回転に使うなど
                        println!("mouse move: ({x},{y}) rel=({dx},{dy})");
                        //TODO
                    },
                    InputEvent::MouseDown { button, x, y } =>
                    {
                        // クリック検知
                        println!("mouse down: {:?} at ({x},{y})", button);
                        //TODO
                    },
                    InputEvent::MouseUp { button, x, y } =>
                    {
                        // リリース検知
                        println!("mouse up: {:?} at ({x},{y})", button);
                        //TODO
                    },
                    InputEvent::MouseWheel { x, y } =>
                    {
                        // 例: y>0で前進、y<0で後退
                        println!("mouse wheel: ({x},{y})");
                        //TODO
                    },

                    InputEvent::TextInput(text) =>
                    {
                        println!("TextInput: {}", text);
                        //TODO
                    },
                    _ => println!("Unhandled event: {:?}", event.type_id()),
                }
            }

            // レンダリングに必要な情報をまとめる
            let render_context = RenderContext {
                frame_index,
                elapsed_seconds: start.elapsed().as_secs_f32(),
            };

            // シーンが変更されていたら、ランタイムシーンを再コンパイルする
            if self.scene_dirty
            {
                self.runtime_scene = self
                    .scene
                    .compile_to_runtime_scene()
                    .map_err(|e| format!("Scene compile failed: {:?}", e))?;
                self.scene_dirty = false;
            }

            // レンダリング開始
            self.renderer.render(
                &mut self.render_target,
                &self.camera,
                &self.runtime_scene,
                &render_context,
            )?;

            // レンダリング結果を画面に表示する
            self.presenter.present(&self.render_target)?;

            frame_index += 1;
            std::thread::sleep(Duration::from_millis(16));
        }
    }

    pub fn get_material_list(&self) -> &[crate::scene::MaterialAsset]
    {
        &self.scene.material_assets()
    }
    
    pub fn get_primitive_list(&self) -> &[crate::scene::PrimitiveAsset]
    {
        &self.scene.primitive_assets()
    }

    pub fn get_instance_list(&self) -> &[crate::scene::Object]
    {
        &self.scene.objects()
    }

    pub fn add_material(&mut self, name: &str, material: crate::scene::Material) -> crate::scene::MaterialId
    {
        let material_id = self.scene.add_material(name, material);
        self.scene_dirty = true;
        material_id
    }

    pub fn add_mesh(&mut self, name: &str, mesh: crate::scene::Mesh) -> crate::scene::PrimitiveId
    {
        let primitive_id = self.scene.add_mesh(name, mesh);
        self.scene_dirty = true;
        primitive_id
    }

    pub fn add_mesh_with_topology(&mut self, name: &str, vertices: Vec<Point>, indices: Vec<[u32; 3]>) -> Result<crate::scene::PrimitiveId, String>
    {
        let primitive_id = self.scene.add_mesh_with_topology(name, vertices, indices).map_err(|e| format!("Failed to add mesh with topology: {:?}", e))?;
        self.scene_dirty = true;
        Ok(primitive_id)
    }

    pub fn add_object(&mut self, object: crate::scene::Object) -> crate::scene::ObjectId
    {
        let object_id = self.scene.add_object(object);
        println!("Added object with ID: {:?}", object_id);
        self.scene_dirty = true;
        object_id
    }



    fn save_scene_data(&self, filename: &str) -> Result<(), String>
    {
        let snap_shot = CameraSnapshot::from_camera(&self.camera);
        println!("Saving scene data to {}: {:?}", filename, snap_shot);
        std::fs::write(filename, serde_json::to_string(&snap_shot).unwrap()).map_err(|e| e.to_string())
    }


}
