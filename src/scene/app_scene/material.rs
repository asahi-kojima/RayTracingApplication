//independent

use crate::internal_prelude::*;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MaterialId(pub(crate) usize);


#[derive(Debug, Clone, PartialEq)]
pub enum Material
{
    Diffuse { albedo: Vec3 },
    Metal { albedo: Vec3, roughness: f64 },
    Dielectric { index_of_refraction: f64 },
    Emissive { color: Vec3, intensity: f64 },
}


#[derive(Debug, Clone)]
pub struct MaterialAsset
{
    name: String,
    material: Material,
}

impl MaterialAsset
{
    pub fn new(name: &str, material: Material) -> Self
    {
        Self { name: name.to_string(), material }
    }

    pub fn name(&self) -> &str
    {
        &self.name
    }

    pub fn material(&self) -> &Material
    {
        &self.material
    }
}
