use ray_tracing::prelude::*;

fn main()
{
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 600;


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

    let mut app = App::new(presenter, camera, 30);

    let mat_id = app.add_material("asahi", Material::Diffuse {
        albedo: Vec3::new(0.8, 0.2, 0.1),
    });

    app.add_object(Object::new(
        "Red Sphere",
        PrimitiveId(0),
        mat_id));
    app.run().unwrap();
}