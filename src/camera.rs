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
}


impl Camera
{
    pub fn new(look_from: Point, look_at: Point, vup: Direction, vfov: f64, aspect_ratio: f64) -> Result<Self, CameraError>
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

        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w: Direction = (look_from - look_at).normalize();
        let u: Direction = vup.cross(w).normalize();
        let v: Direction = w.cross(u).normalize();

        if u.length_squared() < f64::MIN_POSITIVE
        {
            return Err(CameraError::ParallelVup);
        }

        let origin = look_from;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;

        Ok(Self { origin, lower_left_corner, horizontal, vertical })
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

    pub fn get_ray(&self, s: UnitInterval, t: UnitInterval) -> Ray
    {
        let direction = (self.lower_left_corner + s.get() * self.horizontal + t.get() * self.vertical - self.origin).normalize();
        Ray::new(self.origin, direction)
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
        let camera = Camera::new(look_from, look_at, vup, vfov, aspect_ratio).unwrap();
        assert_approx_iter_eq_default(camera.origin(), look_from);
        assert_approx_iter_eq_default(camera.lower_left_corner(), Point::new(-aspect_ratio, -1.0, -1.0));
        assert_approx_iter_eq_default(camera.horizontal(), Vec3::new(2.0 * aspect_ratio, 0.0, 0.0));
        assert_approx_iter_eq_default(camera.vertical(), Vec3::new(0.0, 2.0, 0.0));

        let ray = camera.get_ray(UnitInterval::try_new(0.5).unwrap(), UnitInterval::try_new(0.5).unwrap());
        assert_approx_iter_eq_default(ray.origin(), look_from);
        assert_approx_iter_eq_default(ray.direction(), Direction::new(0.0, 0.0, -1.0));
    }

}