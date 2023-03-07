use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::usize;

use crate::map;
use crate::map::Map;

fn draw_pixel(canvas: &mut Canvas<Window>, x: i32, y: i32) {
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas
        .draw_point(Point::new(x, map::GRID_HEIGHT as i32 - y - 1))
        .unwrap();
}

const COLORS: usize = 4;
const SIZE: usize = map::GRID_WIDTH * map::GRID_HEIGHT * COLORS;

fn grid_to_pixels(grid: &Map) -> Vec<u8> {
    let mut pixel = vec![0; SIZE];

    for (index, colors) in grid
        .grid
        .iter()
        .rev()
        .flatten()
        .map(|b_partical| &b_partical.color)
        .enumerate()
    {
        let step = index * COLORS;

        pixel[step + 0] = colors[0]; // RED
        pixel[step + 1] = colors[1]; // GREEN
        pixel[step + 2] = colors[2]; // BLUE
    }

    pixel
}

pub fn draw_texture(canvas: &mut Canvas<Window>, grid: &Map) {
    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator
        .create_texture(
            PixelFormatEnum::BGR888,
            sdl2::render::TextureAccess::Target,
            map::GRID_WIDTH as u32,
            map::GRID_HEIGHT as u32,
        )
        .unwrap();

    // &[255,255,0,255].repeat(WIDTH * HEIGHT)

    // let now = std::time::Instant::now();
    let pixels = grid_to_pixels(grid);
    // println!("{:.8?},", now.elapsed().as_secs_f32());

    texture
        .update(None, &pixels, map::GRID_WIDTH * 4)
        .expect("Error Updating texture");

    canvas
        .copy(&texture, None, None)
        .expect("Error Copying texture");
}
