use crate::internal_prelude::*;

mod material;
mod object;
mod primitive;
mod geometry_generator;

pub use material::*;
pub use object::*;
pub use primitive::*;
pub use geometry_generator::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct SceneRevision(u64);

mod scene_error;
pub(crate) use scene_error::*;


#[derive(Debug, Clone)]
pub(crate) enum SceneChangeReason
{
    MaterialAdded 
    {
        material_id: MaterialId,
    },
    MaterialChanged 
    {
        material_id: MaterialId,
    },
    PrimitiveAdded 
    {
        primitive_id: PrimitiveId,
    },
    MeshGeometryChanged 
    {
        primitive_id: PrimitiveId,
        vertex_range: std::ops::Range<u32>,
    },
    MeshTopologyChanged 
    {
        primitive_id: PrimitiveId,
    },
    ObjectAdded 
    {
        object_id: ObjectId,
    },
    TransformChanged 
    {
        object_id: ObjectId,
    },
    VisibilityChanged 
    {
        object_id: ObjectId,
    },
    HierarchyChanged 
    {
        object_id: ObjectId,
    },
}


#[derive(Debug, Clone)]
pub(crate) struct SceneChange
{
    revision: SceneRevision,
    reason: SceneChangeReason,
}

impl SceneChange
{
    fn new(revision: SceneRevision, reason: SceneChangeReason) -> Self
    {
        SceneChange{revision, reason}
    }
}


// 人が扱いやすいデータ構造体。レンダリング時には機械が扱いやすい RuntimeScene に変換する必要がある。
#[derive(Debug, Clone, Default)]
pub(crate) struct Scene
{
    primitive_assets: Vec<PrimitiveAsset>,
    material_assets: Vec<MaterialAsset>,
    objects: Vec<Object>,

    revision: SceneRevision,
    changes: Vec<SceneChangeReason>,
    change_history: Vec<SceneChange>
}

impl Scene
{
    pub(crate) fn new() -> Self
    {
        let mut scene = Self::default();
        scene.primitive_assets.push(PrimitiveAsset::new("sphere", PrimitiveId(0), Primitive::Sphere));
        scene.primitive_assets.push(PrimitiveAsset::new("cube", PrimitiveId(1), Primitive::Cube));
        scene.record_change(SceneChangeReason::PrimitiveAdded { primitive_id: PrimitiveId(0) });
        scene.record_change(SceneChangeReason::PrimitiveAdded { primitive_id: PrimitiveId(1) });
        scene
    }

    // --------------------------------------------------------------------------------------------------------------
    // メンバ変数のゲッター
    // --------------------------------------------------------------------------------------------------------------
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

    pub(crate) fn revision(&self) -> SceneRevision
    {
        self.revision
    }


    // --------------------------------------------------------------------------------------------------------------
    // マテリアル操作関数
    // --------------------------------------------------------------------------------------------------------------
    pub(crate) fn add_material(&mut self, name: &str, material: Material) -> MaterialId
    {
        let material_id = MaterialId(self.material_assets.len());
        self.material_assets.push(MaterialAsset::new(name, material));
        
        self.record_change(SceneChangeReason::MaterialAdded { material_id });

        material_id
    }

    pub(crate) fn change_material(&mut self, material_id: MaterialId, new_material: Material) -> Result<(), SceneError>
    {
        let old_material: &mut MaterialAsset = self.material_assets.get_mut(material_id.0).ok_or(MaterialError::UnknownMaterial { material_id })?;
        *old_material = MaterialAsset::new(old_material.name(), new_material);

        self.record_change(SceneChangeReason::MaterialChanged { material_id });

        Ok(())
    }


    // ========================================================================================================
    // メッシュ操作関数
    // ========================================================================================================
    /// シーンにメッシュ(Vertex&Indexの状態で)を追加する
    pub(crate) fn add_primitive_mesh(&mut self, name: &str, mesh: Mesh) -> PrimitiveId
    {
        let primitive_id = PrimitiveId(self.primitive_assets.len());
        self.primitive_assets.push(PrimitiveAsset::new(name, primitive_id, Primitive::Mesh(mesh)));

        self.record_change(SceneChangeReason::PrimitiveAdded { primitive_id });
       
        primitive_id
    }

    pub(crate) fn change_primitive_mesh_geometry(&mut self, primitive_id: PrimitiveId, start: usize, new_vertices: &[Vertex]) -> Result<(), SceneError>
    {
        let old_primitive_mesh = self.primitive_assets.get_mut(primitive_id.0).ok_or(PrimitiveError::UnknownPrimitive { primitive_id })?;
        let Primitive::Mesh(mesh) = old_primitive_mesh.primitive_mut() 
        else 
        {
            return Err(PrimitiveError::NotAMesh { primitive_id }.into());
        };

        let end = start + new_vertices.len();

        let destination = mesh.vertices_mut().get_mut(start..end)
            .ok_or(PrimitiveError::VertexRangeOutOfBounds { primitive_id, start, count: mesh.vertices().len(), vertex_count: new_vertices.len() })?;
        destination.copy_from_slice(new_vertices);

        self.record_change(SceneChangeReason::MeshGeometryChanged { primitive_id, vertex_range: start as u32..end as u32 });

        Ok(())
    }

    pub(crate) fn change_primitive_mesh_topology(&mut self, primitive_id: PrimitiveId, new_mesh: Mesh) -> Result<(), SceneError>
    {
        let old_primitive_mesh = self.primitive_assets.get_mut(primitive_id.0).ok_or(PrimitiveError::UnknownPrimitive { primitive_id })?;
        let Primitive::Mesh(mesh) = old_primitive_mesh.primitive_mut() 
        else 
        {
            return Err(PrimitiveError::NotAMesh { primitive_id }.into());
        };
        *mesh = new_mesh;

        self.record_change(SceneChangeReason::MeshTopologyChanged { primitive_id });

        Ok(())
    }


    // ========================================================================================================
    // オブジェクト操作関数
    // ========================================================================================================
    /// Objectを登録
    pub(crate) fn add_object(&mut self, object: Object) -> ObjectId
    {
        let object_id = ObjectId(self.objects.len());
        self.objects.push(object);

        self.record_change(SceneChangeReason::ObjectAdded { object_id });

        object_id
    }


    /// 親オブジェクトの下に子オブジェクトを追加するメソッド
    pub(crate) fn add_child_object(&mut self, parent_id: ObjectId, mut child: Object) -> Result<ObjectId, SceneError>
    {
        // 親を取得して存在確認を行う
        // TODO: もっといい方法があるけど、一旦これでチェック
        {
            self.get_object_mut(parent_id)?;
        }

        // 子にParentIdを設定する
        child.parent = Some(parent_id);

        // 子IDを取得し、シーンのオブジェクトに登録する
        let child_id = ObjectId(self.objects.len());
        self.objects.push(child);

        // 親に子IDを登録する
        let object = self.get_object_mut(parent_id)?;
        object.children.push(child_id);

        self.record_change(SceneChangeReason::HierarchyChanged { object_id: parent_id });
        self.record_change(SceneChangeReason::ObjectAdded { object_id: child_id });

        Ok(child_id)
    }

    // シーン内のオブジェクトのTransformを変更
    pub(crate) fn set_transform(&mut self, object_id: ObjectId, transform: Transform) -> Result<(), SceneError>
    {
        let object = self.get_object_mut(object_id)?;
        object.set_transform(transform);

        self.record_change(SceneChangeReason::TransformChanged { object_id });

        Ok(())
    }


    /// 可視性の変更
    pub(crate) fn change_visibility(&mut self, object_id: ObjectId, is_visible: bool) -> Result<(), SceneError>
    {
        let object = self.get_object_mut(object_id)?;
        object.is_visible = is_visible;

        self.record_change(SceneChangeReason::VisibilityChanged { object_id });

        Ok(())
    }

    
    /// ObjectId を指定して Object の可変参照を取得
    fn get_object_mut(&mut self, object_id: ObjectId) -> Result<&mut Object, ObjectError>
    {
        self.objects.get_mut(object_id.0).ok_or(ObjectError::UnknownObject { object_id })
    }
    
    
    
    pub(crate) fn make_mermaid_graph_of_instance_dependency(&self) -> String
    {
        let mut graph = String::from("graph TD\n");

        graph.push_str("    classDef objClass fill:#e1f5fe,stroke:#0277bd,stroke-width:2px,color:#000;\n");
        graph.push_str("    classDef primClass fill:#fff3e0,stroke:#ef6c00,stroke-width:2px,color:#000;\n");
        graph.push_str("    classDef matClass fill:#e8f5e9,stroke:#2e7d32,stroke-width:2px,color:#000;\n\n");

        // Objectノードの定義
        for (i, obj) in self.objects.iter().enumerate()
        {
            graph.push_str(&format!("    obj{}[(\"Object: {}\")]:::objClass\n", i, obj.name()));
        }

        graph.push_str("\n");

        // PrimitiveAssetノードの定義
        for prim in &self.primitive_assets
        {
            graph.push_str(&format!("    prim{}[\"Primitive: {}\"]:::primClass\n", prim.id().0, prim.name()));
        }

        graph.push_str("\n");

        // MaterialAssetノードの定義
        for (i, mat) in self.material_assets.iter().enumerate()
        {
            graph.push_str(&format!("    mat{}([\"Material: {}\"]):::matClass\n", i, mat.name()));
        }

        graph.push_str("\n");

        // 依存関係（エッジ）の定義
        for (i, obj) in self.objects.iter().enumerate()
        {
            for child_id in &obj.children
            {
                graph.push_str(&format!("    obj{} --> obj{}\n", i, child_id.0));
            }

            let prim_id = obj.primitive().0;
            let mat_id = obj.material_id().0;

            graph.push_str(&format!("    obj{} -.-> prim{}\n", i, prim_id));
            graph.push_str(&format!("    obj{} -.-> mat{}\n", i, mat_id));
        }

        graph
    }


    // --------------------------------------------------------------------------------------------------------------
    // 変更履歴の保存
    // --------------------------------------------------------------------------------------------------------------
    fn record_change(&mut self, change: SceneChangeReason)
    {
        self.revision = SceneRevision(self.revision.0 + 1);
        self.change_history.push(SceneChange::new(self.revision, change.clone()));
        self.changes.push(change);
    }

    // --------------------------------------------------------------------------------------------------------------
    // 変更履歴のクリア
    // --------------------------------------------------------------------------------------------------------------
    fn clear_change(&mut self, change: SceneChangeReason)
    {
        self.revision = SceneRevision(self.revision.0 + 1);
        self.changes.push(change);
    }
}
