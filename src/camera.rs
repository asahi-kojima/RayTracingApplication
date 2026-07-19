use serde::Serialize;
use crate::internal_prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CameraError {
    InvalidVfov(f64),
    InvalidAspectRatio(f64),
    SameLookFromAndLookAt,
    ParallelVup,
}

pub struct Camera
{
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vec3,
    vertical: Vec3,

    forward: Direction,
    right: Direction,
    up: Direction,
    vfov_deg: f64,
    aspect_ratio: f64,
}


impl Camera
{
    pub fn try_new(look_from: Point, look_at: Point, vup: Direction, vfov: f64, aspect_ratio: f64) -> Result<Self, CameraError>
    {
        if !vfov.is_finite() || vfov <= 0.0 || vfov >= 180.0
        {
            return Err(CameraError::InvalidVfov(vfov));
        }

        if !aspect_ratio.is_finite() || aspect_ratio <= 0.0
        {
            return Err(CameraError::InvalidAspectRatio(aspect_ratio));
        }

        if look_from == look_at
        {
            return Err(CameraError::SameLookFromAndLookAt);
        }

    
        let forward: Direction = (look_at - look_from).normalize();
        let right_vec = forward.cross(vup);

        if right_vec.length_squared() < f64::MIN_POSITIVE
        {
            return Err(CameraError::ParallelVup);
        }

        let right: Direction = right_vec.normalize();
        let up: Direction = right.cross(forward).normalize();

        let mut camera = Self {
            origin: look_from,
            lower_left_corner: look_from,
            horizontal: Vec3::zero(),
            vertical: Vec3::zero(),
            forward,
            right,
            up,
            vfov_deg: vfov,
            aspect_ratio,
        };

        camera.rebuild_viewport();
        Ok(camera)
    }

    fn rebuild_viewport(&mut self)
    {
        let theta = self.vfov_deg.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = self.aspect_ratio * viewport_height;

        self.horizontal = viewport_width * self.right;
        self.vertical = viewport_height * self.up;
        self.lower_left_corner = self.origin + self.forward - self.horizontal / 2.0 - self.vertical / 2.0;
    }

    fn re_orthonormalize_axes(&mut self)
    {
        self.forward = Vec3::from(self.forward).normalize();
        self.right = self.forward.cross(self.up).normalize();
        self.up = self.right.cross(self.forward).normalize();
    }

   pub fn origin(&self) -> Point
    {
        self.origin
    }

    pub fn lower_left_corner(&self) -> Point
    {
        self.lower_left_corner
    }

    pub fn horizontal(&self) -> Vec3
    {
        self.horizontal
    }

    pub fn vertical(&self) -> Vec3
    {
        self.vertical
    }

    pub fn forward(&self) -> Direction
    {
        self.forward
    }

    pub fn right(&self) -> Direction
    {
        self.right
    }

    pub fn up(&self) -> Direction
    {
        self.up
    }

    pub fn vfov_deg(&self) -> f64
    {
        self.vfov_deg
    }

    pub fn aspect_ratio(&self) -> f64
    {
        self.aspect_ratio
    }

    pub fn translate_world(&mut self, delta: Vec3)
    {
        self.origin += delta;
        self.rebuild_viewport();
    }

    pub fn move_forward(&mut self, distance: f64)
    {
        self.origin += self.forward * distance;
        self.rebuild_viewport();
    }

    pub fn move_right(&mut self, distance: f64)
    {
        self.origin += self.right * distance;
        self.rebuild_viewport();
    }

    pub fn move_up(&mut self, distance: f64)
    {
        self.origin += self.up * distance;
        self.rebuild_viewport();
    }

    pub fn yaw(&mut self, radians: f64)
    {
        let q = Quaternion::from_axis_angle(self.up, radians).expect("valid axis");
        self.forward = q.rotate_direction(self.forward);
        self.right = q.rotate_direction(self.right);
        self.re_orthonormalize_axes();
        self.rebuild_viewport();
    }

    pub fn pitch(&mut self, radians: f64)
    {
        let q = Quaternion::from_axis_angle(self.right, radians).expect("valid axis");
        self.forward = q.rotate_direction(self.forward);
        self.up = q.rotate_direction(self.up);
        self.re_orthonormalize_axes();
        self.rebuild_viewport();
    }

    pub fn set_vfov_deg(&mut self, vfov_deg: f64) -> Result<(), CameraError>
    {
        if !vfov_deg.is_finite() || vfov_deg <= 0.0 || vfov_deg >= 180.0
        {
            return Err(CameraError::InvalidVfov(vfov_deg));
        }
        self.vfov_deg = vfov_deg;
        self.rebuild_viewport();
        Ok(())
    }

    pub fn set_aspect_ratio(&mut self, aspect_ratio: f64) -> Result<(), CameraError>
    {
        if !aspect_ratio.is_finite() || aspect_ratio <= 0.0
        {
            return Err(CameraError::InvalidAspectRatio(aspect_ratio));
        }
        self.aspect_ratio = aspect_ratio;
        self.rebuild_viewport();
        Ok(())
    }

    pub(crate) fn get_ray(&self, s: UnitInterval, t: UnitInterval) -> Ray
    {
        let direction =
            (self.lower_left_corner + s.get() * self.horizontal + t.get() * self.vertical
                - self.origin)
                .normalize();
        Ray::new(self.origin, direction)
    }
}


#[derive(Debug, Serialize)]
pub(crate) struct CameraSnapshot
{
    origin: [f64; 3],
    forward: [f64; 3],
    up: [f64; 3],
    right: [f64; 3],
    vfov_deg: f64,
    aspect_ratio: f64,
}

impl CameraSnapshot
{
    pub(crate) fn from_camera(camera: &Camera) -> Self
    {
        Self {
            origin: [camera.origin().x(), camera.origin().y(), camera.origin().z()],
            forward: [camera.forward().x(), camera.forward().y(), camera.forward().z()],
            up: [camera.up().x(), camera.up().y(), camera.up().z()],
            right: [camera.right().x(), camera.right().y(), camera.right().z()],
            vfov_deg: camera.vfov_deg(),
            aspect_ratio: camera.aspect_ratio(),
        }
    }
}

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn test_new_camera_and_get_ray()
    {
        let look_from = Point::new(0.0, 0.0, 0.0);
        let look_at = Point::new(0.0, 0.0, -1.0);
        let vup = Direction::new(0.0, 1.0, 0.0);
        let vfov = 90.0;
        let aspect_ratio = 16.0 / 9.0;
        let camera = Camera::try_new(look_from, look_at, vup, vfov, aspect_ratio).unwrap();
        assert_approx_iter_eq_default(camera.origin(), look_from);
        assert_approx_iter_eq_default(camera.lower_left_corner(), Point::new(-aspect_ratio, -1.0, -1.0));
        assert_approx_iter_eq_default(camera.horizontal(), Vec3::new(2.0 * aspect_ratio, 0.0, 0.0));
        assert_approx_iter_eq_default(camera.vertical(), Vec3::new(0.0, 2.0, 0.0));

        let ray = camera.get_ray(UnitInterval::try_new(0.5).unwrap(), UnitInterval::try_new(0.5).unwrap());
        assert_approx_iter_eq_default(ray.origin(), look_from);
        assert_approx_iter_eq_default(ray.direction(), Direction::new(0.0, 0.0, -1.0));
    }

}