use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::usize;

use crate::grid;

fn draw_pixel(canvas: &mut Canvas<Window>, x: i32, y: i32) {
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas
        .draw_point(Point::new(x, grid::GRID_HEIGHT as i32 - y - 1))
        .unwrap();
}

fn grid_to_pixels(grid: &grid::Grid) -> Vec<u8> {
    // 4 is choosen because RGBA has four Values that need to be traversed.
    const COLORS: usize = 4;
    // This is the array that will be sent to the GPU to render
    // an image. Its in the  formate of RGBA.
    // The type is u8 so we can use values from 0 to 255
    let mut pixel: Vec<u8> = vec![0; (grid::GRID_WIDTH * grid::GRID_HEIGHT * COLORS) as usize];

    // From there we iterate over every partical and set the corrisponding
    // index in the array according to the type of partical selected.
    for (i, ele) in grid
        .grid
        .iter()
        .flatten()
        .map(|f| f.partical_type)
        .enumerate()
    {
        match ele {
            grid::ParticalType::Air => {
                pixel[i * COLORS] = 255; // RED
                pixel[i * COLORS + 1] = 255; // GREEN
                pixel[i * COLORS + 2] = 255; // BLUE
                                             // pixel[i * COLORS + 3] = 000;  ALPHA
            }
            grid::ParticalType::Sand => {
                pixel[i * COLORS] = 192;
                pixel[i * COLORS + 1] = 178;
                pixel[i * COLORS + 2] = 128;
                // pixel[i * COLORS + 3] = 000;
            }

            grid::ParticalType::Rock => {
                pixel[i * COLORS] = 135;
                pixel[i * COLORS + 1] = 135;
                pixel[i * COLORS + 2] = 135;
            }
            grid::ParticalType::Water => {
                pixel[i * COLORS] = 0;
                pixel[i * COLORS + 1] = 50;
                pixel[i * COLORS + 2] = 255;
            }
            grid::ParticalType::Wood => {
                pixel[i * COLORS] = 54;
                pixel[i * COLORS + 1] = 38;
                pixel[i * COLORS + 2] = 27;
            }
            grid::ParticalType::Fire => {
                pixel[i * COLORS] = 226;
                pixel[i * COLORS + 1] = 88;
                pixel[i * COLORS + 2] = 34;
            }
            grid::ParticalType::Smoke => {
                pixel[i * COLORS] = 115;
                pixel[i * COLORS + 1] = 130;
                pixel[i * COLORS + 2] = 118;
            },
        }
    }

    pixel
}

pub fn draw_texture(canvas: &mut Canvas<Window>, grid: &grid::Grid) {
    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator
        .create_texture(
            PixelFormatEnum::BGR888,
            sdl2::render::TextureAccess::Target,
            grid::GRID_WIDTH as u32,
            grid::GRID_HEIGHT as u32,
        )
        .unwrap();

    // &[255,255,0,255].repeat(WIDTH * HEIGHT)
    texture
        .update(None, &grid_to_pixels(grid), grid::GRID_WIDTH * 4)
        .expect("Error Updating texture");

    canvas
        .copy(&texture, None, None)
        .expect("Error Copying texture");
}
