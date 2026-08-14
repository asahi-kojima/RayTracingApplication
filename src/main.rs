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
        Point::new(0.0, 0.0, 10.0),
        Point::new(0.0, 0.0, -1.0),
        Direction::try_new(0.0, 1.0, 0.0).unwrap(),
        90.0,
        WIDTH as f64 / HEIGHT as f64,
    )
    .unwrap();

    let mut app = App::new(renderer, presenter, camera, 30);


    // materialの準備
    let diffuse_material_id = app.add_material("asahi", Material::Diffuse {
        albedo: Vec3::new(0.8, 0.2, 0.1),
    });

    let metal_material_id = app.add_material("metal", Material::Metal { albedo: Vec3::new(1.0, 0.9, 0.8), roughness: 0.0 });

    // meshの準備
    let tetrahedron_mesh: Mesh = generate_tetrahedron();
    let tetrahedron_id = app.add_mesh("tetrahedron", tetrahedron_mesh);


    let primitive_ids = app.get_primitive_list();
    let sphere_primitive_id = primitive_ids[0].id();
    let cube_primitive_id = primitive_ids[1].id();

    app.add_object(Object::new(
        "Red Sphere",
        sphere_primitive_id,
        diffuse_material_id));

    {
        let object_id = app.add_object(Object::new(
            "Blue tetra",
            tetrahedron_id,
            metal_material_id));


        let mut transform = Transform::identity();
        transform = transform.with_position(Point::new(3.0, 0.0, 0.0));

        app.set_transform(object_id, transform);
    }
    {
        let object_id = app.add_object(Object::new(
            "Blue cube",
            cube_primitive_id,
            metal_material_id));


        let mut transform = Transform::identity();
        transform = transform.with_position(Point::new(-3.0, 10.0, 0.0));

        app.set_transform(object_id, transform);
    }

    app.run().unwrap();
}


