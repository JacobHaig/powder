#![feature(slice_group_by)]

use map::{Partical, ParticalType};
use sdl2::pixels::Color;

use std::{mem::size_of, time::Duration};

use crate::event::Input;

mod event;
mod map;
mod render;
mod sim;

#[macro_use]
extern crate num_derive;

static mut PARTICAL_TYPE_VALUE: u32 = 1;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window(
            "sander",
            map::WINDOW_WIDTH as u32,
            map::WINDOW_HEIGHT as u32,
        )
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut input = Input {
        cursur_pos: None,
        mouse_down: false,
        quit: false,
        keys: vec![],
    };

    let mut event_pump = sdl_context.event_pump().unwrap();

    // const INIT: Box<Partical> = None;

    let mut map = map::Map {
        grid: vec![vec![Partical::new(ParticalType::Air); map::GRID_WIDTH]; map::GRID_HEIGHT],
    };

    dbg!(map.grid.len(), map.grid[0].len());

    dbg!(size_of::<map::ParticalType>());
    dbg!(size_of::<map::Partical>());
    dbg!(size_of::<map::Map>());

    'running: loop {
        event::get_mouse_event(&mut event_pump, &mut input);

        match input {
            Input { quit: true, .. } => break 'running,
            Input {
                cursur_pos: Some(pos),
                mouse_down: true,
                ..
            } => circle_partical(&mut map, pos.x, map::GRID_HEIGHT as i32 - pos.y),
            _ => {}
        }

        for key in &input.keys {
            if key.to_string() == "1" {
                unsafe {
                    PARTICAL_TYPE_VALUE += 1;
                }
            }
        }
        input.keys.clear();

        sim::update(&mut map);

        // let now = std::time::Instant::now();
        // println!("Elapsed: {:.8?}", now.elapsed().as_secs_f32());
        render::draw_texture(&mut canvas, &map);

        canvas.present();

        std::thread::sleep(Duration::from_secs_f32(1.0 / 100.0));
    }
}

fn single_partical(grid: &mut map::Map, x: i32, y: i32) {
    let partical_type = unsafe { partical_from_u32(PARTICAL_TYPE_VALUE) };
    let partical = Partical::new(partical_type);

    grid.set_at(x as isize, y as isize, partical);
}

fn circle_partical(grid: &mut map::Map, x: i32, y: i32) {
    let radius = 10;

    for yy in -radius..radius {
        for xx in -radius..radius {
            if xx * xx + yy * yy < radius * radius {
                if let Some(p) = grid.get_at((x + xx) as isize, (y + yy) as isize) {
                    let partical_type = unsafe { partical_from_u32(PARTICAL_TYPE_VALUE) };
                    let partical = Partical::new(partical_type);

                    if p.partical_type == ParticalType::Air {
                        grid.set_at((x + xx) as isize, (y + yy) as isize, partical);
                    }
                }
            }
        }
    }
}

fn partical_from_u32(value: u32) -> map::ParticalType {
    let element = num::FromPrimitive::from_u32(value);
    match element {
        Some(e) => e,
        None => {
            unsafe { PARTICAL_TYPE_VALUE = 0 };
            map::ParticalType::Air
        }
    }
}
