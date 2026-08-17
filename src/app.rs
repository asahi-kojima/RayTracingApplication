use std::any::Any;
use std::time::{Duration, Instant};

use crate::{internal_prelude::*, scene};
use crate::camera::{Camera, CameraSnapshot};
use crate::input::{InputEvent, Key};
use crate::scene::{ObjectId, RuntimeScene, Scene, SceneCompiler, SceneRevision};
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
    compiled_scene_revision: SceneRevision,
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
        let compiled_scene_revision = scene.revision();

        Self {
            renderer,
            presenter,
            camera,
            scene,
            runtime_scene,
            compiled_scene_revision,
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

        let mut fps_timer = Instant::now();
        let mut fps_counter = 0_u32;

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

                            // オブジェクト、プリミティブ、マテリアルの依存関係を出力
                            Key::O =>
                            {
                                println!("Save scene dependency");

                                let parent = std::path::Path::new("storage");
                                let filename = parent.join("dependency.md");

                                std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;

                                let dependency_graph: String = self.scene.make_mermaid_graph_of_instance_dependency();
                                let output = "```mermaid\n".to_string() + &dependency_graph + "```\n";
                                std::fs::write(filename, output).map_err(|e| e.to_string())?;
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
                    _ => println!("--- Unhandled event --- : {:?}", event.type_id()),
                }
            }

            // ----------------------------------------------------------------
            // カメラの移動
            // ----------------------------------------------------------------
            {
                let effective_move_speed = MOVE_SPEED * delta_time.as_secs_f64();
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
                elapsed_seconds: start.elapsed().as_secs_f64(),
            };

            // ----------------------------------------------------------------
            // シーンが変更されていたら、ランタイムシーンを再コンパイルする
            // ----------------------------------------------------------------
            if self.compiled_scene_revision != self.scene.revision()
            {
                self.runtime_scene = SceneCompiler::compile(&self.scene)
                    .map_err(|e| format!("Scene compile failed: {:?}", e))?;
                self.compiled_scene_revision = self.scene.revision();
                self.scene.clear_change();
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

            fps_counter += 1;
            let elapsed_for_fps = fps_timer.elapsed();
            if elapsed_for_fps >= Duration::from_secs(1)
            {
                let fps = fps_counter as f64 / elapsed_for_fps.as_secs_f64();
                println!("FPS: {:.1}", fps);
                fps_counter = 0;
                fps_timer = Instant::now();
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
        material_id
    }

    pub fn add_primitive_mesh(&mut self, name: &str, mesh: crate::scene::Mesh) -> crate::scene::PrimitiveId
    {
        let primitive_id = self.scene.add_primitive_mesh(name, mesh);
        primitive_id
    }

    pub fn add_primitive_mesh_with_topology(&mut self, name: &str, vertices: Vec<Vertex>, indices: Vec<[u32; 3]>) -> Result<crate::scene::PrimitiveId, MathError>
    {
        let mesh = crate::scene::Mesh::try_new(vertices, indices)?;
        let primitive_id = self.scene.add_primitive_mesh(name, mesh);
        Ok(primitive_id)
    }

    pub fn add_object(&mut self, object: crate::scene::Object) -> crate::scene::ObjectId
    {
        let object_id = self.scene.add_object(object);
        println!("Added object with ID: {:?}", object_id);
        object_id
    }

    pub fn add_child_object(&mut self, parent_id: ObjectId, child: crate::scene::Object) -> Result<ObjectId, AppError>
    {
        Ok(self.scene.add_child_object(parent_id, child)?)
    }

    pub fn set_transform(&mut self, object_id: ObjectId, transform: Transform) -> Result<(), AppError>
    {
        Ok(self.scene.set_transform(object_id, transform)?)
    }

    pub fn change_visibility(&mut self, object_id: ObjectId, is_visible: bool)-> Result<(), AppError>
    {
        Ok(self.scene.change_visibility(object_id, is_visible)?)
    }



    fn save_scene_data(&self, filename: &str) -> Result<(), String>
    {
        let snap_shot = CameraSnapshot::from_camera(&self.camera);
        println!("Saving scene data to {}: {:?}", filename, snap_shot);
        std::fs::write(filename, serde_json::to_string(&snap_shot).unwrap()).map_err(|e| e.to_string())
    }


}



#[derive(Debug)]
pub enum AppError
{
    ObjectNotFound
    {
        object_id: usize,
    },
    PrimitiveNotFound 
    {
        primitive_id: usize,
    },
    PrimitiveIsNotMesh 
    {
        primitive_id: usize,
    },
    VertexRangeOutOfBounds 
    {
        primitive_id: usize,
        start: usize,
        count: usize,
        vertex_count: usize,
    },
    InvalidScene 
    {
        message: String,
    },
    RenderFailed 
    {
        message: String,
    },
    IoFailed 
    {
        message: String,
    },
}


impl std::fmt::Display for AppError
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self
        {
            Self::ObjectNotFound { object_id } =>
                write!(formatter, "オブジェクト {} が見つかりません", object_id),

            Self::PrimitiveNotFound { primitive_id } =>
                write!(formatter, "プリミティブ {} が見つかりません", primitive_id),

            Self::PrimitiveIsNotMesh { primitive_id } =>
                write!(formatter, "プリミティブ {} はメッシュではありません", primitive_id),

            Self::VertexRangeOutOfBounds {
                primitive_id,
                start,
                count,
                vertex_count,
            } =>
                write!(
                    formatter,
                    "メッシュ {} の頂点範囲 {}..{} は無効です。頂点数は {} です",
                    primitive_id,
                    start,
                    start + count,
                    vertex_count,
                ),

            Self::InvalidScene { message } =>
                write!(formatter, "シーンが不正です: {}", message),

            Self::RenderFailed { message } =>
                write!(formatter, "描画に失敗しました: {}", message),

            Self::IoFailed { message } =>
                write!(formatter, "ファイル操作に失敗しました: {}", message),
        }
    }
}

impl std::error::Error for AppError {}


impl From<scene::SceneError> for AppError
{
    fn from(error: scene::SceneError) -> Self
    {
        match error
        {
            scene::SceneError::Object(scene::ObjectError::UnknownObject { object_id }) =>
                AppError::ObjectNotFound {
                    object_id: object_id.0,
                },

            scene::SceneError::Primitive(scene::PrimitiveError::UnknownPrimitive { primitive_id }) =>
                AppError::PrimitiveNotFound {
                    primitive_id: primitive_id.0,
                },

            scene::SceneError::Primitive(scene::PrimitiveError::NotAMesh { primitive_id }) =>
                AppError::PrimitiveIsNotMesh {
                    primitive_id: primitive_id.0,
                },

            scene::SceneError::Primitive(
                scene::PrimitiveError::VertexRangeOutOfBounds {
                    primitive_id,
                    start,
                    count,
                    vertex_count,
                },
            ) =>
                AppError::VertexRangeOutOfBounds {
                    primitive_id: primitive_id.0,
                    start,
                    count,
                    vertex_count,
                },

            _ =>
                AppError::InvalidScene {
                    message: format!("{:?}", error),
                },
        }
    }
}