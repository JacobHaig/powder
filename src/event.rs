use sdl2::{event::Event, rect::Point};

use crate::map;

pub struct Input {
    pub cursur_pos: Option<Point>,
    pub mouse_down: bool,
    pub quit: bool,
    pub keys: Vec<sdl2::keyboard::Keycode>,
}

pub fn get_mouse_event(event_pump: &mut sdl2::EventPump, input: &mut Input) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } => input.quit = true,

            Event::MouseButtonDown { x, y, .. } => {
                input.mouse_down = true;

                let x = (x as f32 * map::GRID_WIDTH as f32 / map::WINDOW_WIDTH as f32) as i32;
                let y = (y as f32 * map::GRID_HEIGHT as f32 / map::WINDOW_HEIGHT as f32) as i32;

                input.cursur_pos = Some(Point::new(x, y));
            }

            Event::KeyDown { keycode, .. } => {
                input.keys = vec![keycode.unwrap()]; // Todo: change push keys
            }
            Event::KeyUp { .. } => {
                input.keys = vec![]; // Todo: change push keys
            }

            Event::MouseButtonUp { .. } => input.mouse_down = false,

            Event::MouseMotion {
                x, y, mousestate, ..
            } => {
                if mousestate.is_mouse_button_pressed(sdl2::mouse::MouseButton::Left) {
                    let x = (x as f32 * map::GRID_WIDTH as f32 / map::WINDOW_WIDTH as f32) as i32;
                    let y = (y as f32 * map::GRID_HEIGHT as f32 / map::WINDOW_HEIGHT as f32) as i32;
                    input.cursur_pos = Some(Point::new(x, y));
                }
            }
            _ => {}
        }
    }
}
