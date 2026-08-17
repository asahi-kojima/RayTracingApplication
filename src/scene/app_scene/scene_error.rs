use super::{MaterialId, PrimitiveId, ObjectId};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MaterialError
{
    UnknownMaterial 
    {
        material_id: MaterialId,
    },
}

impl From<MaterialError> for SceneError
{
        fn from(error: MaterialError) -> Self
    {
        Self::Material(error)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum PrimitiveError
{
    UnknownPrimitive 
    {
        primitive_id: PrimitiveId,
    },
    NotAMesh 
    {
        primitive_id: PrimitiveId,
    },
    VertexRangeOutOfBounds 
    {
        primitive_id: PrimitiveId,
        start: usize,
        count: usize,
        vertex_count: usize,
    },
    EmptyVertexUpdate,
    InvalidTopology 
    {
        primitive_id: PrimitiveId,
    },
}

impl From<PrimitiveError> for SceneError
{
    fn from(error: PrimitiveError) -> Self
    {
        Self::Primitive(error)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectError
{
    UnknownObject 
    {
        object_id: ObjectId,
    },
    InvalidParent 
    {
        object_id: ObjectId,
    },
}

impl From<ObjectError> for SceneError
{
    fn from(error: ObjectError) -> Self
    {
        Self::Object(error)
    }
}


#[derive(Debug)]
pub(crate) enum SceneError
{
    Primitive(PrimitiveError),
    Material(MaterialError),
    Object(ObjectError),
}
