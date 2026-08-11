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