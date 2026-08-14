use crate::internal_prelude::*;

mod material;
mod object;
mod primitive;
mod geometry_generator;

pub use material::*;
pub use object::*;
pub use primitive::*;
pub use geometry_generator::*;



// 人が扱いやすいデータ構造体。レンダリング時には機械が扱いやすい RuntimeScene に変換する必要がある。
#[derive(Debug, Clone, Default)]
pub(crate) struct Scene
{
    primitive_assets: Vec<PrimitiveAsset>,
    material_assets: Vec<MaterialAsset>,
    objects: Vec<Object>,
}

impl Scene
{
    pub(crate) fn new() -> Self
    {
        let mut scene = Self::default();
        scene.primitive_assets.push(PrimitiveAsset::new("sphere", PrimitiveId(0), Primitive::Sphere));
        scene.primitive_assets.push(PrimitiveAsset::new("cube", PrimitiveId(1), Primitive::Cube));
        scene
    }

    // -------------------------------------------------------
    // メンバ変数のゲッター
    // -------------------------------------------------------
    pub(crate) fn material_assets(&self) -> &[MaterialAsset]
    {
        &self.material_assets
    }

    pub(crate) fn primitive_assets(&self) -> &[PrimitiveAsset]
    {
        &self.primitive_assets
    }

    pub(crate) fn objects(&self) -> &[Object]
    {
        &self.objects
    }


    // -------------------------------------------------------
    // シーンにマテリアルを追加する
    // -------------------------------------------------------
    pub(crate) fn add_material(&mut self, name: &str, material: Material) -> MaterialId
    {
        let material_id = MaterialId(self.material_assets.len());
        self.material_assets.push(MaterialAsset::new(name, material));
        
        material_id
    }

    // -------------------------------------------------------
    // シーンにメッシュを追加する
    // -------------------------------------------------------
    pub(crate) fn add_mesh(&mut self, name: &str, mesh: Mesh) -> PrimitiveId
    {
        let primitive_id = PrimitiveId(self.primitive_assets.len());
        self.primitive_assets.push(PrimitiveAsset::new(name, primitive_id, Primitive::Mesh(mesh)));
       
        primitive_id
    }

    // -------------------------------------------------------
    // シーンにトポロジーからメッシュを追加する
    // -------------------------------------------------------
    pub(crate) fn add_mesh_with_topology(&mut self, name: &str, vertices: Vec<Vertex>, indices: Vec<[u32; 3]>) -> Result<PrimitiveId, MathError>
    {
        let mesh = Mesh::try_new(vertices, indices)?;
        let primitive_id = self.add_mesh(name, mesh);
        Ok(primitive_id)
    }

    // -------------------------------------------------------
    // ルートオブジェクトとして追加するメソッド
    // -------------------------------------------------------
    pub(crate) fn add_object(&mut self, object: Object) -> ObjectId
    {
        let object_id = ObjectId(self.objects.len());
        self.objects.push(object);

        object_id
    }

    // -------------------------------------------------------
    // 親オブジェクトの下に子オブジェクトを追加するメソッド
    // -------------------------------------------------------
    pub(crate) fn add_child_object(&mut self, parent_id: ObjectId, mut child: Object) -> ObjectId
    {
        self.check_object_id_validation(parent_id);

        // 子にParentIdを設定する
        child.parent = Some(parent_id);

        // 子IDを取得し、シーンのオブジェクトに登録する
        let child_id = ObjectId(self.objects.iter().len());
        self.objects.push(child);

        // 親に子IDを登録する
        let object = self.get_object_mut(parent_id);
        object.children.push(child_id);


        child_id
    }

    // -------------------------------------------------------
    // シーン内のオブジェクトのTransformを変更
    // -------------------------------------------------------
    pub(crate) fn set_transform(&mut self, object_id: ObjectId, transform: Transform)
    {
        self.check_object_id_validation(object_id);
        let mut object = self.get_object_mut(object_id);
        object.set_transform(transform);
    }


    // -------------------------------------------------------
    // -------------------------------------------------------
    pub(crate) fn mesh_vertices_mut(&mut self, primitive_id: PrimitiveId) -> Option<&mut Vec<Vertex>>
    {
        match self.primitive_assets.get_mut(primitive_id.0)?.primitive_mut()
        {
            Primitive::Mesh(mesh) => Some(mesh.vertices_mut()),
            _ => None,
        }
    }

    // -------------------------------------------------------
    // -------------------------------------------------------
    pub(crate) fn change_visibility(&self, object_id: ObjectId, is_visible: bool)
    {
        self.check_object_id_validation(object_id);

    }

    // -------------------------------------------------------
    // 指定されたObjectIdの有効性チェック
    // -------------------------------------------------------
    fn check_object_id_validation(&self, object_id: ObjectId)
    {
        if object_id.0 >= self.objects.len()
        {
            panic!("designated object id is invalid!");
        }
    }

    // -------------------------------------------------------
    // ObjectId を指定して Object の参照を取得
    // -------------------------------------------------------
    fn get_object(&self, object_id: ObjectId) -> &Object
    {
        self.check_object_id_validation(object_id);
        unsafe{self.objects.get_unchecked(object_id.0)}
    }

    // -------------------------------------------------------
    // ObjectId を指定して Object の可変参照を取得
    // -------------------------------------------------------
    fn get_object_mut(&mut self, object_id: ObjectId) -> &mut Object
    {
        self.check_object_id_validation(object_id);
        unsafe{self.objects.get_unchecked_mut(object_id.0)}
    }
}
