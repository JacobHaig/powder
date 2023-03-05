use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::usize;

use crate::map;
use crate::map::ParticalType;

fn draw_pixel(canvas: &mut Canvas<Window>, x: i32, y: i32) {
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas
        .draw_point(Point::new(x, map::GRID_HEIGHT as i32 - y - 1))
        .unwrap();
}

const COLORS: usize = 4;
const SIZE: usize = map::GRID_WIDTH * map::GRID_HEIGHT * COLORS;

fn grid_to_pixels(grid: &map::Map) -> [u8; SIZE] {
    let mut pixel: [u8; SIZE] = [0; SIZE];

    for (i, ele) in grid
        .grid
        .iter()
        .rev()
        .flatten()
        .map(|f| {
            if *f == 0 {
                &ParticalType::Air
            } else {
                &grid.get_partical(*f).unwrap().partical_type
            }
        })
        .enumerate()
    {
        match ele {
            map::ParticalType::Air => {
                pixel[i * COLORS] = 255; // RED
                pixel[i * COLORS + 1] = 255; // GREEN
                pixel[i * COLORS + 2] = 255; // BLUE
                                             // pixel[i * COLORS + 3] = 000;  ALPHA
            }
            map::ParticalType::Sand => {
                pixel[i * COLORS] = 192;
                pixel[i * COLORS + 1] = 178;
                pixel[i * COLORS + 2] = 128;
                // pixel[i * COLORS + 3] = 000;
            }

            map::ParticalType::Rock => {
                pixel[i * COLORS] = 135;
                pixel[i * COLORS + 1] = 135;
                pixel[i * COLORS + 2] = 135;
            }
            map::ParticalType::Water => {
                pixel[i * COLORS] = 0;
                pixel[i * COLORS + 1] = 50;
                pixel[i * COLORS + 2] = 255;
            }
            map::ParticalType::Wood => {
                pixel[i * COLORS] = 54;
                pixel[i * COLORS + 1] = 38;
                pixel[i * COLORS + 2] = 27;
            }
            map::ParticalType::Fire => {
                pixel[i * COLORS] = 226;
                pixel[i * COLORS + 1] = 88;
                pixel[i * COLORS + 2] = 34;
            }
            map::ParticalType::Smoke => {
                pixel[i * COLORS] = 115;
                pixel[i * COLORS + 1] = 130;
                pixel[i * COLORS + 2] = 118;
            }
        }
    }

    pixel
}

pub fn draw_texture(canvas: &mut Canvas<Window>, grid: &map::Map) {
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

    let pixels = grid_to_pixels(grid);

    texture
        .update(None, &pixels, map::GRID_WIDTH * 4)
        .expect("Error Updating texture");

    canvas
        .copy(&texture, None, None)
        .expect("Error Copying texture");
}
