use crate::internal_prelude::*;
use std::cell::Cell;

#[derive(Debug, Clone)]
pub struct Transform
{
    position: Point,
    rotation: Quaternion,
    scale: Vec3,

    is_dirty: Cell<bool>,
    cached_matrix: Cell<Mat4>,
    cached_inverse_matrix: Cell<Mat4>,
    cached_inverse_transpose_matrix: Cell<Mat4>,
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
            cached_matrix: Cell::new(Mat4::identity()),
            cached_inverse_matrix: Cell::new(Mat4::identity()),
            cached_inverse_transpose_matrix: Cell::new(Mat4::identity())
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

    pub fn position(&self) -> Point { self.position }
    pub fn rotation(&self) -> Quaternion { self.rotation }
    pub fn scale(&self) -> Vec3 { self.scale }
    pub fn transform_matrix(&self) -> Mat4 {self.cached_matrix.get()}
    pub fn inv_transform_matrix(&self) -> Mat4 {self.cached_inverse_matrix.get()}
    pub fn inv_transpose_transform_matrix(&self) -> Mat4 {self.cached_inverse_transpose_matrix.get()}

    pub fn with_position(mut self, position: Point) -> Self
    {
        self.position = position;
        self.is_dirty.set(true);
        self
    }

    pub fn with_rotation(mut self, rotation: Quaternion) -> Self
    {
        self.rotation = rotation;
        self.is_dirty.set(true);
        self
    }

    pub fn with_scale(mut self, scale: Vec3) -> Self
    {
        self.scale = scale;
        self.is_dirty.set(true);
        self
    }

    pub fn with_translated(self, offset: Direction) -> Self
    {
        let mut transform = self;
        transform.position += offset;
        transform.is_dirty.set(true);
        transform
    }

    pub fn with_rotated(self, delta: Quaternion) -> Self
    {
        let mut transform = self;
        transform.rotation = delta * transform.rotation;
        transform.is_dirty.set(true);
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
        transform.is_dirty.set(true);
        transform
    }


    pub fn set_position(&mut self, position: Point) -> &mut Self
    {
        self.position = position;
        self.is_dirty.set(true);
        self
    }


    pub fn update_transform_matrices(&self)
    {
        if self.is_dirty.get()
        {
            let transform_matrix: Mat4                  = self.calc_transform_matrix();
            let inverse_transform_matrix: Mat4          = self.calc_inverse_transform_matrix();
            let invese_transpose_transform_matrix: Mat4 = self.calc_inverse_transpose_transform_matrix();

            self.cached_matrix.set(transform_matrix);
            self.cached_inverse_matrix.set(inverse_transform_matrix);
            self.cached_inverse_transpose_matrix.set(invese_transpose_transform_matrix);

            self.is_dirty.set(false);
        }
    }

    fn calc_transform_matrix(&self) -> Mat4
    {
        let s: Mat4 = Mat4::generate_scaling_matrix(self.scale);
        let r: Mat4 = self.rotation.into();
        let t: Mat4 = Mat4::generate_translation_matrix(self.position);

        t * r * s
    }

    fn calc_inverse_transform_matrix(&self) -> Mat4
    {
        let inv_t: Mat4 = Mat4::generate_inverse_translation_matrix(self.position);
        let inv_r: Mat4 = self.rotation.conjugate().into();
        let inv_s: Mat4 = Mat4::generate_inverse_scaling_matrix(self.scale);

        inv_s * inv_r * inv_t
    }

    fn calc_inverse_transpose_transform_matrix(&self) -> Mat4
    {
        let inv_transpose_s: Mat4 = Mat4::generate_inverse_scaling_matrix(self.scale);
        let inv_transpose_r: Mat4 = self.rotation.into();
        let inv_transpose_t: Mat4 = Mat4::generate_inverse_translation_matrix(self.position).transpose();

        inv_transpose_t * inv_transpose_r * inv_transpose_s
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