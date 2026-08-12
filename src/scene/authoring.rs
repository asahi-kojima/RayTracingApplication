use crate::internal_prelude::*;

mod material;
mod object;
mod primitive;

pub use material::*;
pub use object::*;
pub use primitive::*;




// 人が扱いやすいデータ構造体。レンダリング時には機械が扱いやすい RuntimeScene に変換する必要がある。
#[derive(Debug, Clone, Default)]
pub struct Scene
{
    primitive_assets: Vec<PrimitiveAsset>,
    material_assets: Vec<MaterialAsset>,
    objects: Vec<Object>,
}

impl Scene
{
    pub fn new() -> Self
    {
        let mut scene = Self::default();
        scene.primitive_assets.push(PrimitiveAsset::new("sphere", PrimitiveId(0), Primitive::Sphere));
        scene.primitive_assets.push(PrimitiveAsset::new("cube", PrimitiveId(1), Primitive::Cube));
        scene
    }

    pub fn material_assets(&self) -> &[MaterialAsset]
    {
        &self.material_assets
    }

    pub fn primitive_assets(&self) -> &[PrimitiveAsset]
    {
        &self.primitive_assets
    }

    pub fn objects(&self) -> &[Object]
    {
        &self.objects
    }

    pub fn add_material(&mut self, name: &str, material: Material) -> MaterialId
    {
        let material_id = MaterialId(self.material_assets.len());
        self.material_assets.push(MaterialAsset::new(name, material));
        material_id
    }

    pub fn add_mesh(&mut self, name: &str, mesh: Mesh) -> PrimitiveId
    {
        let primitive_id = PrimitiveId(self.primitive_assets.len());
        self.primitive_assets.push(PrimitiveAsset::new(name, primitive_id, Primitive::Mesh(mesh)));
        primitive_id
    }

    pub fn add_mesh_with_topology(&mut self, name: &str, vertices: Vec<Point>, indices: Vec<[u32; 3]>) -> Result<PrimitiveId, MathError>
    {
        let mesh = Mesh::try_new(vertices, indices)?;
        let primitive_id = self.add_mesh(name, mesh);
        Ok(primitive_id)
    }

    // ルートオブジェクトとして追加するメソッド
    pub fn add_object(&mut self, object: Object) -> ObjectId
    {
        let object_id = ObjectId(self.objects.len());
        self.objects.push(object);
        object_id
    }

    // 親オブジェクトの下に子オブジェクトを追加するメソッド
    pub fn add_child_object(&mut self, parent_id: ObjectId, mut child: Object) -> Result<ObjectId, &'static str>
    {
        if parent_id.0 >= self.objects().len()
        {
            return Err("Invalid Parent ObjectID");
        }

        // 子にParentIdを設定する
        child.parent = Some(parent_id);

        // 子IDを取得し、シーンのオブジェクトに登録する
        let child_id = ObjectId(self.objects.iter().len());
        self.objects.push(child);

        // 親に子IDを登録する
        if let Some(object) = self.get_object_mut(parent_id)
        {
            object.children.push(child_id);
            Ok(child_id)
        }
        else
        {
            return Err("failed to get parent object");
        }
    }


    pub fn mesh_vertices_mut(&mut self, primitive_id: PrimitiveId) -> Option<&mut Vec<Point>>
    {
        match self.primitive_assets.get_mut(primitive_id.0)?.primitive_mut()
        {
            Primitive::Mesh(mesh) => Some(mesh.vertices_mut()),
            _ => None,
        }
    }

    pub fn compile_to_runtime_scene(&self) -> Result<super::RuntimeScene, super::runtime_scene::SceneCompileError>
    {
        super::runtime_scene::SceneCompiler::compile(self)
    }

    // ObjectId を指定して Object の参照を取得
    pub fn get_object(&self, id: ObjectId) -> Option<&Object> 
    {
        self.objects.get(id.0)
    }

    // ObjectId を指定して Object の可変参照を取得
    pub fn get_object_mut(&mut self, id: ObjectId) -> Option<&mut Object> 
    {
        self.objects.get_mut(id.0)
    }
}
