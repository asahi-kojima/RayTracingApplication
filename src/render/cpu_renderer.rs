use crate::camera::Camera;
use crate::render::{Frame, RenderContext, Renderer};
use crate::util::UnitInterval;

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
        ctx: &RenderContext,
    ) -> Result<(), String>
    {
        let width = frame.width() as usize;
        let height = frame.height() as usize;

        let denom_x = (width.saturating_sub(1)).max(1) as f64;
        let denom_y = (height.saturating_sub(1)).max(1) as f64;

        let pixels = frame.pixels_mut();

        let sun_dx = 0.795;
        let sun_dy = 0.556;
        let sun_dz = -0.246;

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

                    let sun_dot = (dir.x() * sun_dx + dir.y() * sun_dy + dir.z() * sun_dz).max(0.0);
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
