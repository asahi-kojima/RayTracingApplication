use std::any::Any;
use std::time::{Duration, Instant};

use crate::internal_prelude::*;
use crate::camera::{Camera, CameraSnapshot};
use crate::input::{InputEvent, Key};
use crate::scene::{ObjectId, RuntimeScene, Scene, SceneCompiler};
use crate::platform::Presenter;
use crate::render::{Frame, RenderContext, Renderer, CpuRenderer, GpuRenderer};
use crate::scene::Vertex;

const MOVE_SPEED: f64 = 1.0;
const ROT_SPEED_RAD: f64 = 3.0_f64.to_radians();


#[derive(Debug, Clone, Copy, Default)]
struct InputState
{
    pub is_w_pressed: bool,
    pub is_s_pressed: bool,
    pub is_a_pressed: bool,
    pub is_d_pressed: bool,

    pub is_right_pressed: bool,
    pub is_left_pressed : bool,
    pub is_up_pressed   : bool,
    pub is_down_pressed : bool,
}


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
        let runtime_scene = SceneCompiler::compile(&scene).expect("empty scene compile should always succeed");

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

        let target_frame_duration = Duration::from_secs_f64(1.0 / self.fps as f64);

        // 前フレームからの経過時間
        let mut delta_time = Duration::from_secs(0);
        
        // このフレームの開始時刻
        let mut frame_start_time = Instant::now();

        // 開始時点からの経過フレーム
        let mut frame_index = 0_u64;

        let mut input_state = InputState::default();

        loop
        {
            // ----------------------------------------------------------------
            // 時間の計測と更新
            // ----------------------------------------------------------------
            let now = Instant::now();
            delta_time = now.duration_since(frame_start_time);
            frame_start_time = now;


            // ----------------------------------------------------------------
            // イベント処理
            // ----------------------------------------------------------------
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
                            Key::W => input_state.is_w_pressed = true,
                            Key::S => input_state.is_s_pressed = true,
                            Key::A => input_state.is_a_pressed = true,
                            Key::D => input_state.is_d_pressed = true,
                            
                            // 矢印: 視点回転
                            Key::LEFT  => input_state.is_left_pressed  = true,
                            Key::RIGHT => input_state.is_right_pressed = true,
                            Key::UP    => input_state.is_up_pressed    = true,
                            Key::DOWN  => input_state.is_down_pressed  = true,

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
                    InputEvent::KeyUp(key) =>
                    {
                        match key
                        {

                            // WASD: 平行移動
                            Key::W => input_state.is_w_pressed = false,
                            Key::S => input_state.is_s_pressed = false,
                            Key::A => input_state.is_a_pressed = false,
                            Key::D => input_state.is_d_pressed = false,
                            
                            // 矢印: 視点回転
                            Key::LEFT  => input_state.is_left_pressed  = false,
                            Key::RIGHT => input_state.is_right_pressed = false,
                            Key::UP    => input_state.is_up_pressed    = false,
                            Key::DOWN  => input_state.is_down_pressed  = false,

                            _ => println!("no action"),
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

            // ----------------------------------------------------------------
            // カメラの移動
            // ----------------------------------------------------------------
            {
                let effective_move_speed = MOVE_SPEED * delta_time.as_secs_f64();println!("{}", effective_move_speed);
                let effective_rot_speed_rad = ROT_SPEED_RAD * delta_time.as_secs_f64();

                if input_state.is_w_pressed { self.camera.move_forward(effective_move_speed); }
                if input_state.is_s_pressed { self.camera.move_forward(-effective_move_speed); }
                if input_state.is_a_pressed { self.camera.move_right(-effective_move_speed); }
                if input_state.is_d_pressed { self.camera.move_right(effective_move_speed); }
                
                if input_state.is_left_pressed  { self.camera.yaw(effective_rot_speed_rad); }
                if input_state.is_right_pressed { self.camera.yaw(-effective_rot_speed_rad); }
                if input_state.is_up_pressed    { self.camera.pitch(effective_rot_speed_rad); }
                if input_state.is_down_pressed  { self.camera.pitch(-effective_rot_speed_rad); }
            }

            // ----------------------------------------------------------------
            // レンダリングに必要な情報をまとめる
            // ----------------------------------------------------------------
            let render_context = RenderContext {
                frame_index,
                elapsed_seconds: start.elapsed().as_secs_f32(),
            };

            // ----------------------------------------------------------------
            // シーンが変更されていたら、ランタイムシーンを再コンパイルする
            // ----------------------------------------------------------------
            if self.scene_dirty
            {
                self.runtime_scene = SceneCompiler::compile(&self.scene)
                    .map_err(|e| format!("Scene compile failed: {:?}", e))?;
                self.scene_dirty = false;
            }

            // ----------------------------------------------------------------
            // レンダリング開始
            // ----------------------------------------------------------------
            self.renderer.render(
                &mut self.render_target,
                &self.camera,
                &self.runtime_scene,
                &render_context,
            )?;

            // ----------------------------------------------------------------
            // レンダリング結果を画面に表示する
            // ----------------------------------------------------------------
            self.presenter.present(&self.render_target)?;

            // ----------------------------------------------------------------
            // FPSの制御
            // ----------------------------------------------------------------
            frame_index += 1;
            let frame_work_duration = frame_start_time.elapsed();
            if frame_work_duration < target_frame_duration
            {
                let sleep_time = target_frame_duration - frame_work_duration;

                if sleep_time > Duration::from_millis(2)
                {
                    std::thread::sleep(sleep_time - Duration::from_millis(2));
                }

                while frame_start_time.elapsed() < target_frame_duration
                {
                    std::hint::spin_loop();
                }
            }
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

    pub fn add_mesh_with_topology(&mut self, name: &str, vertices: Vec<Vertex>, indices: Vec<[u32; 3]>) -> Result<crate::scene::PrimitiveId, String>
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

    pub fn set_transform(&mut self, object_id: ObjectId, transform: Transform)
    {
        self.scene.set_transform(object_id, transform);
    }



    fn save_scene_data(&self, filename: &str) -> Result<(), String>
    {
        let snap_shot = CameraSnapshot::from_camera(&self.camera);
        println!("Saving scene data to {}: {:?}", filename, snap_shot);
        std::fs::write(filename, serde_json::to_string(&snap_shot).unwrap()).map_err(|e| e.to_string())
    }


}
