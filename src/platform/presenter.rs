use crate::render::Frame;
use crate::input::InputEvent;

pub trait Presenter
{
    fn size(&self) -> (u32, u32);

    fn handle_events(&mut self, events: &mut Vec<InputEvent>) -> bool;

    fn present(&mut self, frame: &Frame) -> Result<(), String>;
}
