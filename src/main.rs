#![feature(slice_group_by)]

use rand::Rng;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::mem::size_of;
use std::time::Duration;

mod grid;
mod render;
mod sim;

#[macro_use]
extern crate num_derive;

static mut PARTICAL_TYPE_VALUE: u32 = 0;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window(
            "sander",
            grid::WINDOW_WIDTH as u32,
            grid::WINDOW_HEIGHT as u32,
        )
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut mouse_down = false;
    let mut cursur_pos = Point::new(0, 0);
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut grid = grid::Grid {
        grid: vec![vec![grid::Partical::new(); grid::GRID_WIDTH]; grid::GRID_HEIGHT],
        cell_changes: Vec::new(),
    };

    dbg!(size_of::<grid::Partical>());
    dbg!(size_of::<grid::Grid>());

    //let mut fps = Vec::new();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::MouseButtonDown { x, y, .. } => {
                    mouse_down = true;
                    cursur_pos.x =
                        (x as f32 * grid::GRID_WIDTH as f32 / grid::WINDOW_WIDTH as f32) as i32;
                    cursur_pos.y =
                        (y as f32 * grid::GRID_HEIGHT as f32 / grid::WINDOW_HEIGHT as f32) as i32;
                }
                Event::KeyDown { keycode, .. } => {
                    if keycode.unwrap().to_string() == "1" {
                        unsafe {
                            PARTICAL_TYPE_VALUE += 1;
                        }
                    }
                }
                Event::MouseButtonUp { .. } => mouse_down = false,
                Event::MouseMotion {
                    x, y, mousestate, ..
                } => {
                    if mousestate.is_mouse_button_pressed(sdl2::mouse::MouseButton::Left) {
                        cursur_pos.x =
                            (x as f32 * grid::GRID_WIDTH as f32 / grid::WINDOW_WIDTH as f32) as i32;
                        cursur_pos.y = (y as f32 * grid::GRID_HEIGHT as f32
                            / grid::WINDOW_HEIGHT as f32)
                            as i32;
                    }
                }
                _ => {}
            }
        }

        if mouse_down {
            //single_partical(&mut grid, cursur_pos.x, cursur_pos.y);
            //rect_partical(&mut grid, cursur_pos.x, cursur_pos.y);
            circle_partical(&mut grid, cursur_pos.x, cursur_pos.y);
        }

        //let start1 = std::time::Instant::now();
        sim::simulate_points(&mut grid);
        sim::update(&mut grid);
        // let time1 = start1.elapsed();

        // let start2 = std::time::Instant::now();
        render::draw_texture(&mut canvas, &grid);
        // let time2 = start2.elapsed();

        // let total = start1.elapsed();

        // fps.push(1.0 / (total.as_secs_f32()));

        /*println!(
            "sim {:?} - draw {:?} - total {:?} - Ave FPS {:?} - FPS {:?}",
            time1,
            time2,
            &total,
            fps.iter().rev().take(100).sum::<f32>() / fps.iter().rev().take(100).len() as f32,
            fps.iter().last()
        );*/

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 300));
    }
}

fn single_partical(grid: &mut grid::Grid, x: i32, y: i32) {
    let mut new = grid::Partical::new();
    new.partical_type = grid::ParticalType::Sand;
    // grid.set_at(x as usize, HEIGHT - y as usize, new);
    grid.set_at(x as isize, y as isize, new);
}

fn rect_partical(grid: &mut grid::Grid, x: i32, y: i32) {
    let mut new = grid::Partical::new();

    //new.partical_type = random_partical(0, 4);
    new.partical_type = unsafe { partical_from_u32(PARTICAL_TYPE_VALUE) };

    let radius = 10;

    for yy in -radius..radius {
        for xx in -radius..radius {
            //grid.set_at((x + xx) as usize, (HEIGHT as i32 - y + yy) as usize, new);
            grid.set_at((x + xx) as isize, (y + yy) as isize, new);
        }
    }
}

fn circle_partical(grid: &mut grid::Grid, x: i32, y: i32) {
    let mut new = grid::Partical::new();

    //new.partical_type = random_partical(0, 4);
    new.partical_type = unsafe { partical_from_u32(PARTICAL_TYPE_VALUE) };

    let radius = 50;

    // Equation for a circle.
    for yy in -radius..radius {
        for xx in -radius..radius {
            // x^2 + y^2 = r^2
            if xx * xx + yy * yy < radius * radius {
                grid.set_at((x + xx) as isize, (y + yy) as isize, new);
            }
        }
    }
}

fn partical_from_u32(value: u32) -> grid::ParticalType {
    let element = num::FromPrimitive::from_u32(value);
    match element {
        Some(e) => e,
        None => {
            unsafe { PARTICAL_TYPE_VALUE = 0 };
            grid::ParticalType::Air
        }
    }
}

fn random_partical(low: u32, high: u32) -> grid::ParticalType {
    let mut rng = rand::thread_rng();
    let num: u32 = rng.gen_range(low..high);
    dbg!(num);

    let element = num::FromPrimitive::from_u32(num);
    match element {
        Some(e) => e,
        None => unreachable!(),
    }
}
