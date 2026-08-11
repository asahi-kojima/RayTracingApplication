use crate::internal_prelude::*;
use super::primitive::*;
use super::material::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObjectId(pub(crate) usize);


#[derive(Debug, Clone)]
pub struct Object
{
    name: String,
    primitive_id: PrimitiveId,
    material_id: MaterialId,
    transform: Transform,
}

impl Object
{
    pub fn new(name: &str, primitive_id: PrimitiveId, material_id: MaterialId) -> Self
    {
        Self {
            name: name.to_string(),
            primitive_id,
            material_id,
            transform: Transform::identity(),
        }
    }

    pub fn name(&self) -> &str
    {
        &self.name
    }

    pub fn primitive(&self) -> PrimitiveId
    {
        self.primitive_id
    }

    pub fn material_id(&self) -> MaterialId
    {
        self.material_id
    }


    pub fn transform(&self) -> &Transform
    {
        &self.transform
    }

    pub fn set_transform(&mut self, transform: Transform)
    {
        self.transform = transform;
    }
}