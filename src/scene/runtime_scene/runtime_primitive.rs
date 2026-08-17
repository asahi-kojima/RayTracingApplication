#[derive(Debug, Clone, PartialEq)]
pub(crate) struct RuntimeMeshRef
{
	pub vertex_range: std::ops::Range<u32>,  // primitive_vertices バッファ内の頂点スライス
	pub indices: Vec<[u32; 3]>,              // vertex_range.start からの相対インデックス. vertex_rangeに足して使うこと
}



#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate)  struct RuntimePrimitiveId(pub(crate) usize);



#[derive(Debug, Clone, PartialEq)]
pub(crate) enum RuntimePrimitive
{
	SphereUnit,
	CubeUnit,
	MeshTriangles(RuntimeMeshRef),
}

