//independent

use crate::internal_prelude::*;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PrimitiveId(pub(crate) usize);



#[derive(Debug, Clone)]
pub struct PrimitiveAsset
{
    name: String,
    id: PrimitiveId,
    primitive: Primitive,
}

impl PrimitiveAsset
{
    pub fn new(name: &str, id: PrimitiveId, primitive: Primitive) -> Self
    {
        Self { name: name.to_string(), id, primitive }
    }

    pub fn name(&self) -> &str
    {
        &self.name
    }

    pub fn primitive(&self) -> &Primitive
    {
        &self.primitive
    }

    pub fn id(&self) -> PrimitiveId
    {
        self.id
    }

    pub fn primitive_mut(&mut self) -> &mut Primitive
    {
        &mut self.primitive
    }
}


#[derive(Debug, Clone)]
pub enum Primitive
{
    Sphere,
    Cube,
    Mesh(Mesh),
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vertex
{
    point: Point,
    normal: Direction
}

impl Vertex
{
    pub fn new(point: Point, normal: Direction) -> Self
    {
        Self{point, normal}
    }

    pub fn point(&self) -> Point {self.point}

    pub fn norma(&self) -> Direction {self.normal}
}

#[derive(Debug, Clone, PartialEq)]
pub struct Mesh
{
    vertices: Vec<Vertex>,
    indices: Vec<[u32; 3]>,
}

impl Mesh
{
    pub(crate) fn new(vertices: Vec<Vertex>, indices: Vec<[u32; 3]>) -> Self
    {
        Self { vertices, indices }
    }

    pub fn try_new(vertices: Vec<Vertex>, indices: Vec<[u32; 3]>) -> Result<Self, MathError>
    {
        // 各三角形の法線ベクトルを計算して、面積が0の三角形がないかチェックする
        for [i0, i1, i2] in &indices
        {
            let _triangle = Triangle::try_new(
                vertices[*i0 as usize].point,
                vertices[*i1 as usize].point,
                vertices[*i2 as usize].point,
            )?;
        }
        Ok(Self { vertices, indices })
    }

    pub fn vertices(&self) -> &[Vertex]
    {
        &self.vertices
    }

    pub fn vertices_mut(&mut self) -> &mut Vec<Vertex>
    {
        &mut self.vertices
    }

    pub fn indices(&self) -> &[[u32; 3]]
    {
        &self.indices
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triangle
{
    v0: Point,
    v1: Point,
    v2: Point,

    normal: Direction,
}

impl Triangle
{
    pub(crate) fn new(v0: Point, v1: Point, v2: Point) -> Self
    {
        let edge1 = v1 - v0;
        let edge2 = v2 - v0;
        let normal = edge1.cross(edge2).normalize();
        Self { v0, v1, v2, normal }
    }

    pub fn try_new(v0: Point, v1: Point, v2: Point) -> Result<Self, MathError>
    {
        let edge1 = v1 - v0;
        let edge2 = v2 - v0;
        let normal = edge1.cross(edge2).try_normalize()?;
        Ok(Self { v0, v1, v2, normal })
    }

    pub fn v0(&self) -> Point
    {
        self.v0
    }

    pub fn v1(&self) -> Point
    {
        self.v1
    }

    pub fn v2(&self) -> Point
    {
        self.v2
    }
}
