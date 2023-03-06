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

    for (index, ele) in grid
        .grid
        .iter()
        .rev()
        .flatten()
        .map(|b_partical| &b_partical.partical_type)
        .enumerate()
    {
        let step = index * COLORS;

        match ele {
            map::ParticalType::Air => {
                pixel[step] = 255; // RED
                pixel[step + 1] = 255; // GREEN
                pixel[step + 2] = 255; // BLUE
                                       // pixel[i * COLORS + 3] = 000;  ALPHA
            }
            map::ParticalType::Sand => {
                pixel[step] = 192;
                pixel[step + 1] = 178;
                pixel[step + 2] = 128;
                // pixel[i * COLORS + 3] = 000;
            }

            map::ParticalType::Rock => {
                pixel[step] = 135;
                pixel[step + 1] = 135;
                pixel[step + 2] = 135;
            }
            map::ParticalType::Water => {
                pixel[step] = 0;
                pixel[step + 1] = 50;
                pixel[step + 2] = 255;
            }
            map::ParticalType::Wood => {
                pixel[step] = 54;
                pixel[step + 1] = 38;
                pixel[step + 2] = 27;
            }
            map::ParticalType::Fire => {
                pixel[step] = 226;
                pixel[step + 1] = 88;
                pixel[step + 2] = 34;
            }
            map::ParticalType::Smoke => {
                pixel[step] = 115;
                pixel[step + 1] = 130;
                pixel[step + 2] = 118;
            }
        }
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
