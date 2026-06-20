use crate::internal_prelude::*;
use std::cell::Cell;

#[derive(Debug, Clone)]
pub struct Transform
{
    position: Point,
    rotation: Quaternion,
    scale: Vec3,

    is_dirty: Cell<bool>,
    cached_matrix: Cell<Option<Mat4>>,
    cached_inverse_matrix: Cell<Option<Mat4>>,
}

impl Transform
{
    pub fn new(position: Point, rotation: Quaternion, scale: Vec3) -> Self
    {
        Self
        {
            position,
            rotation,
            scale,
            is_dirty: Cell::new(true),
            cached_matrix: Cell::new(None),
            cached_inverse_matrix: Cell::new(None),
        }
    }

    pub fn identity() -> Self
    {
        Self::new(
            Point::new(0.0, 0.0, 0.0),
            Quaternion::identity(),
            Vec3::one(),
        )
    }

    fn invalidate_cache(&self)
    {
        self.is_dirty.set(true);
        self.cached_matrix.set(None);
        self.cached_inverse_matrix.set(None);
    }

    pub fn position(&self) -> Point { self.position }
    pub fn rotation(&self) -> Quaternion { self.rotation }
    pub fn scale(&self) -> Vec3 { self.scale }

    pub fn with_position(mut self, position: Point) -> Self
    {
        self.position = position;
        self.invalidate_cache();
        self
    }

    pub fn with_rotation(mut self, rotation: Quaternion) -> Self
    {
        self.rotation = rotation;
        self.invalidate_cache();
        self
    }

    pub fn with_scale(mut self, scale: Vec3) -> Self
    {
        self.scale = scale;
        self.invalidate_cache();
        self
    }

    pub fn with_translated(self, offset: Direction) -> Self
    {
        let mut transform = self;
        transform.position += offset;
        transform.invalidate_cache();
        transform
    }

    pub fn with_rotated(self, delta: Quaternion) -> Self
    {
        let mut transform = self;
        transform.rotation = delta * transform.rotation;
        transform.invalidate_cache();
        transform
    }

    pub fn with_scaled(self, factor: Vec3) -> Self
    {
        let mut transform = self;
        transform.scale = Vec3::new(
            transform.scale.x() * factor.x(),
            transform.scale.y() * factor.y(),
            transform.scale.z() * factor.z(),
        );
        transform.invalidate_cache();
        transform
    }


    pub fn set_position(&mut self, position: Point) -> &mut Self
    {
        self.position = position;
        self.invalidate_cache();
        self
    }
}

impl Default for Transform
{
    fn default() -> Self
    {
        Self::identity()
    }
}


#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    pub fn test_transform()
    {
        let transform = Transform::identity()
            .with_position(Point::new(1.0, 2.0, 3.0))
            .with_rotation(Quaternion::try_from_axis_angle(Vec3::unit_y(), 90.0_f64.to_radians()).unwrap())
            .with_scale(Vec3::new(2.0, 2.0, 2.0));

        assert!(transform.position() == Point::new(1.0, 2.0, 3.0));
        assert!(transform.rotation() == Quaternion::try_from_axis_angle(Vec3::unit_y(), 90.0_f64.to_radians()).unwrap());
        assert!(transform.scale() == Vec3::new(2.0, 2.0, 2.0));
    }
}