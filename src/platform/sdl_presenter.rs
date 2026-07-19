use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::{EventPump, Sdl};

use sdl2::mouse::MouseButton as SdlMouseButton;
use crate::platform::Presenter;
use crate::render::Frame;
use crate::input::{Key, InputEvent, MouseButton};

fn map_keycode(keycode: Keycode) -> Key
{
    match keycode 
    {
        Keycode::W => Key::W,
        Keycode::A => Key::A,
        Keycode::S => Key::S,
        Keycode::D => Key::D,
        Keycode::Q => Key::Q,
        Keycode::P => Key::P,
        Keycode::Left => Key::LEFT,
        Keycode::Right => Key::RIGHT,
        Keycode::Up => Key::UP,
        Keycode::Down => Key::DOWN,
        Keycode::Space => Key::SPACE,
        Keycode::Escape => Key::ESCAPE,
        _ => Key::OTHER,
    }
}


fn map_mouse_button(button: SdlMouseButton) -> MouseButton
{
    match button
    {
        SdlMouseButton::Left => MouseButton::LEFT,
        SdlMouseButton::Right => MouseButton::RIGHT,
        SdlMouseButton::Middle => MouseButton::MIDDLE,
        SdlMouseButton::X1 => MouseButton::X1,
        SdlMouseButton::X2 => MouseButton::X2,
        _ => MouseButton::OTHER,
    }
}

pub struct SdlPresenter
{
    _sdl: Sdl,
    canvas: Canvas<Window>,
    event_pump: EventPump,
    width: u32,
    height: u32,
}

impl SdlPresenter
{
    pub fn try_new(title: &str, width: u32, height: u32) -> Result<Self, String>
    {
        let sdl = sdl2::init()?;
        let video_subsystem = sdl.video()?;

        let window = video_subsystem
            .window(title, width, height)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        let event_pump = sdl.event_pump()?;

        Ok(Self {
            _sdl: sdl,
            canvas,
            event_pump,
            width,
            height,
        })
    }
}

impl Presenter for SdlPresenter
{
    fn size(&self) -> (u32, u32)
    {
        (self.width, self.height)
    }

    fn handle_events(&mut self, events: &mut Vec<InputEvent>) -> bool
    {
        for event in self.event_pump.poll_iter()
        {
            match event
            {
                Event::Quit { .. }=> 
                {
                    events.push(InputEvent::Quit);
                    events.push(InputEvent::KeyDown(Key::Q));
                    return true;
                },
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } =>
                {
                    events.push(InputEvent::KeyDown(Key::ESCAPE));
                    return true;
                },
                Event::KeyDown { keycode: Some(keycode), .. }=>
                {
                    let key = map_keycode(keycode);
                    events.push(InputEvent::KeyDown(key));
                },
                Event::KeyUp { keycode: Some(keycode), .. } => 
                {
                    let key = map_keycode(keycode);
                    events.push(InputEvent::KeyUp(key));
                },
                Event::TextInput { text, .. } =>
                {
                    events.push(InputEvent::TextInput(text));
                },
                Event::MouseMotion { x, y, xrel, yrel, .. } =>
                {
                    events.push(InputEvent::MouseMove {x, y, dx: xrel, dy: yrel});
                },
                Event::MouseButtonDown { mouse_btn, x, y, .. } =>
                {
                    events.push(InputEvent::MouseDown {
                        button: map_mouse_button(mouse_btn), x, y});
                },
                Event::MouseButtonUp { mouse_btn, x, y, .. } =>
                {
                    events.push(InputEvent::MouseUp {
                        button: map_mouse_button(mouse_btn), x, y});
                },
                Event::MouseWheel { x, y, .. } =>
                {
                    events.push(InputEvent::MouseWheel {x, y});
                },
                _ => {}
            }
        }
        false
    }

    fn present(&mut self, frame: &Frame) -> Result<(), String>
    {
        if frame.width() != self.width || frame.height() != self.height
        {
            return Err(format!(
                "Frame size mismatch: frame={}x{}, presenter={}x{}",
                frame.width(),
                frame.height(),
                self.width,
                self.height
            ));
        }

        let texture_creator = self.canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_streaming(PixelFormatEnum::RGB24, self.width, self.height)
            .map_err(|e| e.to_string())?;

        let pitch = (self.width * 3) as usize;
        texture
            .update(None, frame.pixels(), pitch)
            .map_err(|e| e.to_string())?;

        self.canvas.clear();
        self.canvas.copy(&texture, None, None)?;
        self.canvas.present();

        Ok(())
    }
}
