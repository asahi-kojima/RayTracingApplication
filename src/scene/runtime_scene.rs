use crate::internal_prelude::*;
use std::collections::HashMap;
use super::app_scene::{Material, MaterialId, PrimitiveId, ObjectId, Scene, Vertex};


mod runtime_primitive;
mod runtime_transform;
mod runtime_instance;

pub(crate) use runtime_primitive::*;
pub(crate) use runtime_transform::*;
pub(crate) use runtime_instance::*;



// レンダリング時に機械が扱いやすいデータ構造体。CPU/GPU独自処理の前の共通データ構造
#[derive(Debug, Clone)]
pub(crate) struct RuntimeScene
{
	materials: Vec<Material>,
	primitives: Vec<RuntimePrimitive>,
	primitive_vertices: Vec<Vertex>,
	instances: Vec<RuntimeInstance>,

	authoring_to_runtime_primitive: HashMap<PrimitiveId, RuntimePrimitiveId>,
	authoring_to_runtime_object: HashMap<ObjectId, RuntimeInstanceId>,
}

impl RuntimeScene
{
	pub fn new(
		materials: Vec<Material>,
		primitives: Vec<RuntimePrimitive>,
		primitive_vertices: Vec<Vertex>,
		instances: Vec<RuntimeInstance>,
		authoring_to_runtime_primitive: HashMap<PrimitiveId, RuntimePrimitiveId>,
		authoring_to_runtime_object: HashMap<ObjectId, RuntimeInstanceId>,
	) -> Self
	{
		Self {
			materials,
			primitives,
			primitive_vertices,
			instances,
			authoring_to_runtime_primitive,
			authoring_to_runtime_object,
		}
	}

	pub fn materials(&self) -> &[Material]
	{
		&self.materials
	}

	pub fn primitives(&self) -> &[RuntimePrimitive]
	{
		&self.primitives
	}

	pub fn primitive_vertices(&self) -> &[Vertex]
	{
		&self.primitive_vertices
	}

	pub fn instances(&self) -> &[RuntimeInstance]
	{
		&self.instances
	}

	pub fn runtime_primitive_id_of(&self, primitive_id: PrimitiveId) -> Option<RuntimePrimitiveId>
	{
		self.authoring_to_runtime_primitive.get(&primitive_id).copied()
	}

	pub fn runtime_instance_id_of(&self, object_id: ObjectId) -> Option<RuntimeInstanceId>
	{
		self.authoring_to_runtime_object.get(&object_id).copied()
	}

	pub fn update_mesh_vertices(
		&mut self,
		primitive_id: PrimitiveId,
		new_vertices: &[Vertex],
	) -> Result<(), MeshUpdateError>
	{
		let runtime_id = self.authoring_to_runtime_primitive
			.get(&primitive_id)
			.copied()
			.ok_or(MeshUpdateError::UnknownPrimitive(primitive_id))?;

		let mesh_ref = match &self.primitives[runtime_id.0]
		{
			RuntimePrimitive::MeshTriangles(r) => r.clone(),
			_ => return Err(MeshUpdateError::NotAMesh(primitive_id)),
		};

		let expected = (mesh_ref.vertex_range.end - mesh_ref.vertex_range.start) as usize;
		if new_vertices.len() != expected
		{
			return Err(MeshUpdateError::VertexCountMismatch { expected, got: new_vertices.len() });
		}

		let start = mesh_ref.vertex_range.start as usize;
		for (i, &v) in new_vertices.iter().enumerate()
		{
			self.primitive_vertices[start + i] = v;
		}
		Ok(())
	}

	pub fn update_instance_transform(
		&mut self,
		object_id: ObjectId,
		transform: Transform,
	) -> bool
	{
		let Some(instance_id) = self.runtime_instance_id_of(object_id) else
		{
			return false;
		};

		let Some(instance) = self.instances.get_mut(instance_id.0) else
		{
			return false;
		};

		instance.set_transform(RuntimeTransform::from_transform(transform));
		true
	}
}
















#[derive(Debug, Clone, PartialEq)]
pub enum MeshUpdateError
{
	UnknownPrimitive(PrimitiveId),
	NotAMesh(PrimitiveId),
	VertexCountMismatch { expected: usize, got: usize },
}

#[derive(Debug, Clone, PartialEq)]
pub enum SceneCompileError
{
    InvalidMaterialId {
        object_id: ObjectId,
        material_id: MaterialId,
    },
    InvalidPrimitiveId {
        object_id: ObjectId,
        primitive_id: PrimitiveId,
    },
    MeshIndexOutOfRange {
        primitive_id: PrimitiveId,
        triangle_index: usize,
        vertex_index: u32,
        vertex_count: usize,
    },
    DegenerateTriangle {
        primitive_id: PrimitiveId,
        triangle_index: usize,
    },
    NonInvertibleTransform {
        object_id: ObjectId,
    },
}

use super::app_scene::Primitive;
pub(crate) struct SceneCompiler;
impl SceneCompiler
{
    pub(crate) fn compile(scene: &Scene) -> Result<RuntimeScene, SceneCompileError>
    {
        let materials = scene
            .material_assets()
            .iter()
            .map(|asset| asset.material().clone())
            .collect::<Vec<_>>();

		// -----------------------------------------------------------------------------------
		// シーン内のプリミティブをコンパイルして、RuntimePrimitive と RuntimeMeshRef を作成する
		// -----------------------------------------------------------------------------------
        let mut runtime_primitives = Vec::<RuntimePrimitive>::new();
        let mut primitive_vertices = Vec::<Vertex>::new();
        let mut primitive_map = HashMap::<PrimitiveId, RuntimePrimitiveId>::new();

        for primitive_asset in scene.primitive_assets().iter()
        {
            let primitive_id = primitive_asset.id();
            let runtime_primitive_id = RuntimePrimitiveId(runtime_primitives.len());

			// DEBUG & MUST DELETE
			{
				let runtime_id = runtime_primitive_id.0;
				let primitive_id = primitive_asset.id().0;
				if runtime_id != primitive_id
				{
					panic!("Compiling primitive: authoring_id={}, runtime_id={}", primitive_id, runtime_id);
				}
			}

            let runtime_primitive = match primitive_asset.primitive()
            {
                Primitive::Sphere => RuntimePrimitive::SphereUnit,
                Primitive::Cube => RuntimePrimitive::CubeUnit,
                Primitive::Mesh(mesh) => 
				{
                    let start = primitive_vertices.len() as u32;
                    primitive_vertices.extend_from_slice(mesh.vertices());
                    let end = primitive_vertices.len() as u32;

                    RuntimePrimitive::MeshTriangles(RuntimeMeshRef {
                        vertex_range: start..end,
                        indices: mesh.indices().to_vec(),
                    })
                },
            };

            runtime_primitives.push(runtime_primitive);
            primitive_map.insert(primitive_id, runtime_primitive_id);
        }

		// -----------------------------------------------------------------------------------
		// シーン内のインスタンスをコンパイルして、RuntimeInstance を作成する
		// -----------------------------------------------------------------------------------
        let mut instances = Vec::<RuntimeInstance>::new();
        let mut object_map = HashMap::<ObjectId, RuntimeInstanceId>::new();

        for (object_index, object) in scene.objects().iter().enumerate()
        {
            let object_id = ObjectId(object_index);

            if object.material_id().0 >= materials.len()
            {
                return Err(SceneCompileError::InvalidMaterialId {
                    object_id,
                    material_id: object.material_id(),
                });
            }

            let Some(runtime_primitive_id) = primitive_map.get(&object.primitive()).copied() else
            {
                return Err(SceneCompileError::InvalidPrimitiveId{object_id, primitive_id: object.primitive()});
            };

            let scale = object.transform().scale();
            if scale.x().abs() <= 1.0e-12 || scale.y().abs() <= 1.0e-12 || scale.z().abs() <= 1.0e-12
            {
                return Err(SceneCompileError::NonInvertibleTransform { object_id });
            }

            let runtime_instance_id = RuntimeInstanceId(instances.len());
            instances.push(RuntimeInstance::new(
                object.name().to_string(),
                runtime_primitive_id,
                object.material_id(),
                RuntimeTransform::from_transform(object.transform().clone()),
            ));
            object_map.insert(object_id, runtime_instance_id);

        }

        Ok(RuntimeScene::new(
            materials,
            runtime_primitives,
            primitive_vertices,
            instances,
            primitive_map,
            object_map,
        ))
    }
}
