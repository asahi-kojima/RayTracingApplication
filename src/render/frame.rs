#[derive(Debug, Clone)]
pub struct Frame
{
    width: u32,
    height: u32,
    pixels: Vec<u8>,
}

impl Frame
{
    pub(crate) fn new(width: u32, height: u32) -> Self
    {
        Self {
            width,
            height,
            pixels: vec![0; (width * height * 3) as usize],
        }
    }

    pub(crate) fn width(&self) -> u32 {self.width}

    pub(crate) fn height(&self) -> u32 {self.height}

    pub(crate) fn pixels(&self) -> &[u8] {&self.pixels}

    pub(crate) fn pixels_mut(&mut self) -> &mut [u8] {&mut self.pixels}
}
