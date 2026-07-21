use crate::internal_prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaterialId(pub(crate) usize);

#[derive(Debug, Clone, Default)]
pub struct Scene
{
    materials: Vec<Material>,
    objects: Vec<Object>,
}

impl Scene
{
    pub fn new() -> Self
    {
        Self::default()
    }

    pub fn materials(&self) -> &[Material]
    {
        &self.materials
    }

    pub fn objects(&self) -> &[Object]
    {
        &self.objects
    }

    pub fn add_material(&mut self, material: Material) -> MaterialId
    {
        let material_id = MaterialId(self.materials.len());
        self.materials.push(material);
        material_id
    }

    pub fn add_object(&mut self, object: Object) -> usize
    {
        self.objects.push(object);
        self.objects.len() - 1
    }
}

#[derive(Debug, Clone)]
pub struct Object
{
    primitive: Primitive,
    material_id: MaterialId,
    transform: Transform,
}

impl Object
{
    pub fn new(primitive: Primitive, material_id: MaterialId) -> Self
    {
        Self {
            primitive,
            material_id,
            transform: Transform::identity(),
        }
    }

    pub fn primitive(&self) -> &Primitive
    {
        &self.primitive
    }

    pub fn material_id(&self) -> MaterialId
    {
        self.material_id
    }

    pub fn transform(&self) -> &Transform
    {
        &self.transform
    }

    pub fn with_transform(mut self, transform: Transform) -> Self
    {
        self.transform = transform;
        self
    }
}

#[derive(Debug, Clone)]
pub enum Primitive
{
    Sphere(Sphere),
    Triangle(Triangle),
    Mesh(Mesh),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Material
{
    Diffuse { albedo: Vec3 },
    Metal { albedo: Vec3, roughness: f64 },
    Dielectric { index_of_refraction: f64 },
    Emissive { color: Vec3, intensity: f64 },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sphere
{
    center: Point,
    radius: f64,
}

impl Sphere
{
    pub fn new(center: Point, radius: f64) -> Self
    {
        Self { center, radius }
    }

    pub fn center(&self) -> Point
    {
        self.center
    }

    pub fn radius(&self) -> f64
    {
        self.radius
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Mesh
{
    vertices: Vec<Point>,
    indices: Vec<[u32; 3]>,
}

impl Mesh
{
    pub fn try_new(vertices: Vec<Point>, indices: Vec<[u32; 3]>) -> Result<Self, MathError>
    {
        // 各三角形の法線ベクトルを計算して、面積が0の三角形がないかチェックする
        for [i0, i1, i2] in &indices
        {
            let _triangle = Triangle::try_new(
                vertices[*i0 as usize],
                vertices[*i1 as usize],
                vertices[*i2 as usize],
            )?;
        }
        Ok(Self { vertices, indices })
    }

    pub fn vertices(&self) -> &[Point]
    {
        &self.vertices
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


#[derive(Debug, Clone, Copy)]
pub(crate) struct HitRecord
{
    pub(crate) t: f64,
    pub(crate) point: Point,
    pub(crate) normal: Direction,
    pub(crate) front_face: bool,
}

pub(crate) trait Hittable
{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

impl Hittable for Sphere
{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>
    {
        // p = o + td ==> |p - c|^2 <= r^2を解く
        // |d|^2 t^2 + 2 d·oc t + |oc|^2 - r^2 <= 0
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = oc.dot(ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0
        {
            return None;
        }

        let sqrt_discriminant = discriminant.sqrt();
        let mut t = (-half_b - sqrt_discriminant) / a;

        if t < t_min || t > t_max
        {
            t = (-half_b + sqrt_discriminant) / a;
            if t < t_min || t > t_max
            {
                return None;
            }
        }

        let point = ray.at(t);
        let outward_normal = (point - self.center).normalize();
        let front_face = ray.direction().dot(outward_normal) < 0.0;

        Some(HitRecord {
            t,
            point,
            normal: if front_face { outward_normal } else { -outward_normal },
            front_face,
        })
    }
}

impl Hittable for Triangle
{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>
    {
        let edge1 = self.v1 - self.v0;
        let edge2 = self.v2 - self.v0;
        let ray_direction_x_edge2 = ray.direction().cross(edge2);
        let det = edge1.dot(ray_direction_x_edge2);

        // 三角形とレイが平行であるかのチェック: 立体を作っているか？
        if det.abs() < 1e-8
        {
            return None;
        }


        // 三角形の平面上の点は p = v0 + u * edge1 + v * edge2 で表される
        // レイとこの平面の交差点をまず求める: o + td = v0 + u * edge1 + v * edge2
        // ==>  d t - edge1 u - edge2 v = v0 - o
        // (d, -edge1, -edge2) * (t, u, v)^T = v0 - o <== 線形代数で普通に解ける

        let inv_det = 1.0 / det;
        let origin_to_v0 = ray.origin() - self.v0;
        let u = inv_det * origin_to_v0.dot(ray_direction_x_edge2);

        if u < 0.0 || u > 1.0
        {
            return None;
        }

        let origin_cross_edge1 = origin_to_v0.cross(edge1);
        let v = inv_det * ray.direction().dot(origin_cross_edge1);

        // v < 0.0 || v > 1.0 のチェックに加え、内部条件 u + v <= 1.0を考慮すると以下の条件になる
        // u + v <= 1.0 ==> v <= 1.0 - u, ここで 0 <= 1 - u <= 1.0　なので、u + v <= 1.0 は v <= 1.0を包含する
        if v < 0.0 || u + v > 1.0
        {
            return None;
        }

        let t = inv_det * edge2.dot(origin_cross_edge1);

        if t < t_min || t > t_max
        {
            return None;
        }

        let point = ray.at(t);
        let normal = self.normal;
        let front_face = ray.direction().dot(normal) < 0.0;

        Some(HitRecord {
            t,
            point,
            normal: if front_face { normal } else { -normal },
            front_face,
        })
    }
}

impl Hittable for Mesh
{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>
    {
        let mut closest = t_max;
        let mut hit_record = None;

        for [i0, i1, i2] in &self.indices
        {
            let triangle = Triangle::new(
                self.vertices[*i0 as usize],
                self.vertices[*i1 as usize],
                self.vertices[*i2 as usize],
            );

            if let Some(hit) = triangle.hit(ray, t_min, closest)
            {
                closest = hit.t;
                hit_record = Some(hit);
            }
        }

        hit_record
    }
}

impl Hittable for Primitive
{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>
    {
        match self
        {
            Primitive::Sphere(sphere) => sphere.hit(ray, t_min, t_max),
            Primitive::Triangle(triangle) => triangle.hit(ray, t_min, t_max),
            Primitive::Mesh(mesh) => mesh.hit(ray, t_min, t_max),
        }
    }
}