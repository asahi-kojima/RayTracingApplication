use crate::camera::Camera;
use crate::math::{Direction, Point, Transform, Vec3};
use crate::scene::{Material, RuntimeMeshRef, RuntimePrimitive, RuntimeScene, Vertex};
use crate::render::{Frame, RenderContext, Renderer};
use crate::util::UnitInterval;

#[derive(Debug, Clone, Copy)]
struct HitInfo
{
    normal: Direction,
    material_index: usize,
}

// ローカル空間（中心=原点, 半径=1の単位球）でのレイ交差判定。
// world_to_local_vec で変換した local_dir は大きさが変わるが、t値はワールド空間と同じになる。
fn unit_sphere_hit_local(
    local_origin: Vec3,
    local_dir: Vec3,
    t_min: f64,
    t_max: f64,
) -> Option<(f64, Vec3)>  // (t, ローカル法線)
{
    let a = local_dir.dot(local_dir);
    let half_b = local_origin.dot(local_dir);
    let c = local_origin.dot(local_origin) - 1.0;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0
    {
        return None;
    }

    let sqrt_d = discriminant.sqrt();
    let mut t = (-half_b - sqrt_d) / a;
    if t < t_min || t > t_max
    {
        t = (-half_b + sqrt_d) / a;
        if t < t_min || t > t_max
        {
            return None;
        }
    }

    let local_normal = local_origin + local_dir * t;  // 単位球: 法線 = ヒット点
    Some((t, local_normal))
}

// ワールド→ローカルへ点を変換（逆TRS: 逆平行移動 -> 逆回転 -> 逆スケール）
fn world_to_local_point(transform: &Transform, p: Point) -> Vec3
{
    let s = transform.scale();
    let v = Vec3::from(p) - Vec3::from(transform.position());
    let r = transform.rotation().conjugate().rotate_vec3(v);
    Vec3::new(r.x() / s.x(), r.y() / s.y(), r.z() / s.z())
}

// ワールド→ローカルへ方向を変換（平行移動なし、逆回転 -> 逆スケール）
fn world_to_local_vec(transform: &Transform, d: Vec3) -> Vec3
{
    let s = transform.scale();
    let r = transform.rotation().conjugate().rotate_vec3(d);
    Vec3::new(r.x() / s.x(), r.y() / s.y(), r.z() / s.z())
}

// ローカル法線をワールド法線へ変換（逆転置行列 = 回転 * 逆スケール）
fn local_normal_to_world(transform: &Transform, local_n: Vec3) -> Option<Direction>
{
    let s = transform.scale();
    let scaled = Vec3::new(local_n.x() / s.x(), local_n.y() / s.y(), local_n.z() / s.z());
    transform.rotation().rotate_vec3(scaled).try_normalize().ok()
}

fn unit_sphere_hit_instance(
    ray_origin: Point,
    ray_dir: Direction,
    transform: &Transform,
    t_min: f64,
    t_max: f64,
) -> Option<(f64, Direction)>
{
    let local_origin = world_to_local_point(transform, ray_origin);
    let local_dir = world_to_local_vec(transform, Vec3::from(ray_dir));

    let (t, local_normal) = unit_sphere_hit_local(local_origin, local_dir, t_min, t_max)?;

    let outward = local_normal_to_world(transform, local_normal)?;
    let normal = if Vec3::from(ray_dir).dot(Vec3::from(outward)) < 0.0 { outward } else { -outward };
    Some((t, normal))
}

// ローカル空間での Möller-Trumbore。dir は非正規化 Vec3。t はワールド空間と等価。
fn triangle_hit_local(origin: Vec3, dir: Vec3, v0: Point, v1: Point, v2: Point, t_min: f64, t_max: f64) -> Option<f64>
{
    let epsilon = 1.0e-8;
    let edge1 = v1 - v0;
    let edge2 = v2 - v0;
    let pvec = dir.cross(edge2);
    let det = edge1.dot(pvec);

    if det.abs() < epsilon { return None; }

    let inv_det = 1.0 / det;
    let tvec = Vec3::from(v0) - origin;
    let u = (-tvec).dot(pvec) * inv_det;
    if !(0.0..=1.0).contains(&u) { return None; }

    let qvec = (-tvec).cross(edge1);
    let v = dir.dot(qvec) * inv_det;
    if v < 0.0 || u + v > 1.0 { return None; }

    let t = edge2.dot(qvec) * inv_det;
    if t < t_min || t > t_max { return None; }

    Some(t)
}

fn mesh_hit_instance(
    ray_origin: Point,
    ray_dir: Direction,
    transform: &Transform,
    mesh_ref: &RuntimeMeshRef,
    primitive_vertices: &[Vertex],
    t_min: f64,
    t_max: f64,
) -> Option<(f64, Direction)>
{
    let local_origin = world_to_local_point(transform, ray_origin);
    let local_dir = world_to_local_vec(transform, Vec3::from(ray_dir));

    let mut best_t = t_max;
    let mut best_local_normal: Option<Vec3> = None;

    let vstart = mesh_ref.vertex_range.start as usize;
    for &[i0, i1, i2] in &mesh_ref.indices
    {
        let v0 = primitive_vertices[vstart + i0 as usize].point();
        let v1 = primitive_vertices[vstart + i1 as usize].point();
        let v2 = primitive_vertices[vstart + i2 as usize].point();
        if let Some(t) = triangle_hit_local(local_origin, local_dir, v0, v1, v2, t_min, best_t)
        {
            best_t = t;
            best_local_normal = Some((v1 - v0).cross(v2 - v0));
        }
    }

    let local_n = best_local_normal?;
    let outward = local_normal_to_world(transform, local_n)?;
    let normal = if Vec3::from(ray_dir).dot(Vec3::from(outward)) < 0.0 { outward } else { -outward };
    Some((best_t, normal))
}

// スラブ法による単位立方体（[-1,1]^3）との交差判定
fn unit_cube_hit_instance(
    ray_origin: Point,
    ray_dir: Direction,
    transform: &Transform,
    t_min: f64,
    t_max: f64,
) -> Option<(f64, Direction)>
{
    let local_origin = world_to_local_point(transform, ray_origin);
    let local_dir = world_to_local_vec(transform, Vec3::from(ray_dir));

    let mut t_near = t_min;
    let mut t_far = t_max;
    let mut near_axis = 0usize;
    let mut near_sign = 1.0f64;

    for axis in 0..3
    {
        let o = local_origin[axis];
        let d = local_dir[axis];

        if d.abs() < 1.0e-10
        {
            if o < -1.0 || o > 1.0 { return None; }
            continue;
        }

        let inv_d = 1.0 / d;
        let (t1, t2, s1) = if inv_d >= 0.0
        {
            ((-1.0 - o) * inv_d, (1.0 - o) * inv_d, -1.0f64)
        }
        else
        {
            ((1.0 - o) * inv_d, (-1.0 - o) * inv_d, 1.0f64)
        };

        if t1 > t_near { t_near = t1; near_axis = axis; near_sign = s1; }
        if t2 < t_far { t_far = t2; }
        if t_near > t_far { return None; }
    }

    let (t_hit, local_n) = if t_near >= t_min
    {
        let mut n = Vec3::zero();
        match near_axis {
            0 => { let _ = near_sign; n = Vec3::new(near_sign, 0.0, 0.0); },
            1 => { n = Vec3::new(0.0, near_sign, 0.0); },
            _ => { n = Vec3::new(0.0, 0.0, near_sign); },
        }
        (t_near, n)
    }
    else
    {
        return None;
    };

    let outward = local_normal_to_world(transform, local_n)?;
    let normal = if Vec3::from(ray_dir).dot(Vec3::from(outward)) < 0.0 { outward } else { -outward };
    Some((t_hit, normal))
}

fn first_hit(scene: &RuntimeScene, ray_origin: Point, ray_dir: Direction) -> Option<HitInfo>
{
    let mut best_t = f64::INFINITY;
    let mut best_hit: Option<HitInfo> = None;

    for instance in scene.instances()
    {
        let transform = instance.transform().transform();
        let result = match &scene.primitives()[instance.primitive_id().0]
        {
            RuntimePrimitive::SphereUnit =>
                unit_sphere_hit_instance(ray_origin, ray_dir, transform, 1.0e-4, best_t),
            RuntimePrimitive::CubeUnit =>
                unit_cube_hit_instance(ray_origin, ray_dir, transform, 1.0e-4, best_t),
            RuntimePrimitive::MeshTriangles(mesh_ref) =>
                mesh_hit_instance(ray_origin, ray_dir, transform, mesh_ref, scene.primitive_vertices(), 1.0e-4, best_t),
        };

        if let Some((t, normal)) = result
        {
            best_t = t;
            best_hit = Some(HitInfo { normal, material_index: instance.material_id().0 });
        }
    }

    best_hit
}

fn material_base_color(material: &Material) -> Vec3
{
    match material
    {
        Material::Diffuse { albedo } => *albedo,
        Material::Metal { albedo, .. } => *albedo,
        Material::Dielectric { .. } => Vec3::new(0.92, 0.95, 1.0),
        Material::Emissive { color, intensity } => *color * *intensity,
    }
}

fn shade_hit(scene: &RuntimeScene, hit: HitInfo, sun: Direction) -> (f64, f64, f64)
{
    let default_color = Vec3::new(0.8, 0.8, 0.8);
    let base_color = scene
        .materials()
        .get(hit.material_index)
        .map(material_base_color)
        .unwrap_or(default_color);

    let lambert = hit.normal.dot(-sun).max(0.0);
    let ambient = 0.08;
    let lit = base_color * (ambient + 0.92 * lambert);

    (lit.x(), lit.y(), lit.z())
}

pub struct CpuRenderer;

impl CpuRenderer
{
    pub fn new() -> Self
    {
        Self
    }
}

impl Default for CpuRenderer
{
    fn default() -> Self
    {
        Self::new()
    }
}

impl Renderer for CpuRenderer
{
    fn render(
        &mut self,
        frame: &mut Frame,
        camera: &Camera,
        runtime_scene: &RuntimeScene,
        ctx: &RenderContext,
    ) -> Result<(), String>
    {
        let width = frame.width() as usize;
        let height = frame.height() as usize;

        let denom_x = (width.saturating_sub(1)).max(1) as f64;
        let denom_y = (height.saturating_sub(1)).max(1) as f64;

        let pixels = frame.pixels_mut();

        let sun = Direction::try_new(0.795, 0.556, -0.246).expect("sun direction must be valid");

        let ground_y = -0.5_f64;
        let to_u8 = |c: f64| -> u8 { (c.clamp(0.0, 1.0) * 255.999) as u8 };

        for iy in 0..height
        {
            for ix in 0..width
            {
                let u = UnitInterval::try_new(ix as f64 / denom_x).expect("u out of range");
                let v = UnitInterval::try_new(1.0 - (iy as f64 / denom_y)).expect("v out of range");

                let ray = camera.get_ray(u, v);
                let dir = ray.direction();
                let ori = ray.origin();

                if let Some(hit) = first_hit(runtime_scene, ori, dir)
                {
                    let (r, g, b) = shade_hit(runtime_scene, hit, sun);
                    let i = (iy * width + ix) * 3;
                    pixels[i] = to_u8(r);
                    pixels[i + 1] = to_u8(g);
                    pixels[i + 2] = to_u8(b);
                    continue;
                }

                let (r, g, b) = if dir.y() < -1.0e-6
                {
                    // ray と地面平面 y=ground_y の交点
                    let t_hit = (ground_y - ori.y()) / dir.y();

                    if t_hit > 0.0
                    {
                        let p = ray.at(t_hit);

                        // チェッカーパターン
                        let cx = p.x().floor() as i32;
                        let cz = p.z().floor() as i32;
                        let checker = ((cx + cz) & 1) == 0;

                        let base = if checker
                        {
                            (1.0, 0.0, 0.0)
                        }
                        else
                        {
                            (0.0, 1.0, 0.0)
                        };

                        // 距離で少し減衰させて奥行き感
                        let fog = (1.0 / (1.0 + 0.08 * t_hit * t_hit)).clamp(0.0, 1.0);

                        (
                            base.0 * fog + (1.0 - fog) * 0.55,
                            base.1 * fog + (1.0 - fog) * 0.68,
                            base.2 * fog + (1.0 - fog) * 0.90,
                        )
                    }
                    else
                    {
                        // 地面に当たらない場合は空
                        let t = 0.5 * (dir.y() + 1.0);
                        (
                            (1.0 - t) * 1.00 + t * 0.50,
                            (1.0 - t) * 1.00 + t * 0.72,
                            (1.0 - t) * 1.00 + t * 1.00,
                        )
                    }
                }
                else
                {
                    // 上向きレイは空
                    let t = 0.5 * (dir.y() + 1.0);
                    let mut r = (1.0 - t) * 1.00 + t * 0.50;
                    let mut g = (1.0 - t) * 1.00 + t * 0.72;
                    let mut b = (1.0 - t) * 1.00 + t * 1.00;

                    let horizon = (1.0 - (dir.y().abs() * 7.0).min(1.0)) * 0.10;
                    r += horizon;
                    g += horizon;
                    b += horizon * 0.8;

                    let sun_dot = dir.dot(sun).max(0.0);
                    let sun_core = sun_dot.powf(320.0);
                    let sun_glow = sun_dot.powf(24.0) * 0.25;
                    let flicker = 0.9 + 0.1 * (ctx.elapsed_seconds as f64 * 1.7).sin();

                    r += (sun_core * 1.0 + sun_glow * 1.0) * flicker;
                    g += (sun_core * 0.8 + sun_glow * 0.6) * flicker;
                    b += (sun_core * 0.4 + sun_glow * 0.2) * flicker;

                    (r, g, b)
                };

                let i = (iy * width + ix) * 3;
                pixels[i] = to_u8(r);
                pixels[i + 1] = to_u8(g);
                pixels[i + 2] = to_u8(b);
            }
        }

        Ok(())
    }
}
