use ray_tracing::prelude::*;

fn main()
{
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 600;

    let backend = std::env::var("RAY_BACKEND").unwrap_or_else(|_| "cpu".to_string());
    let renderer: Box<dyn Renderer> = match backend.as_str()
    {
        "gpu" => Box::new(GpuRenderer::new()),
        _ => Box::new(CpuRenderer::new()),
    };

    let presenter: Box<dyn Presenter> =
        Box::new(SdlPresenter::try_new("Ray Tracing in Rust", WIDTH, HEIGHT).unwrap());

    let camera = Camera::try_new(
        Point::new(0.0, 0.0, 0.0),
        Point::new(0.0, 0.0, -1.0),
        Direction::try_new(0.0, 1.0, 0.0).unwrap(),
        90.0,
        WIDTH as f64 / HEIGHT as f64,
    )
    .unwrap();

    let mut app = App::new(renderer, presenter, camera, 30);
    app.run().unwrap();
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