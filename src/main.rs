use ray_tracing::prelude::*;

fn main()
{
    println!("Hello, Ray Tracing in Rust!");
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