use rand::Rng;

use crate::map::{self, ParticalType};

// Bresenham's line algorithm - only integer arithmetic
fn line(x0: isize, y0: isize, x1: isize, y1: isize) -> Vec<(isize, isize)> {
    let mut result = Vec::with_capacity(16);

    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();

    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };

    let mut x = x0;
    let mut y = y0;
    let mut err = dx - dy;

    while x != x1 || y != y1 {
        result.push((x, y));
        let e2 = 2 * err;

        if e2 > -dy {
            err -= dy;
            x += sx;
        }

        if e2 < dx {
            err += dx;
            y += sy;
        }
    }

    result.push((x1, y1));
    result
}

// Takes the grid and a position then returns a list of points that
// that partical will travel if not interrupted.
fn apply_force(grid: &mut map::Map, x: isize, y: isize) -> Vec<(isize, isize)> {
    let partical = grid.get_mut_at(x, y).unwrap();

    const GRAVITY: f32 = 1.0;

    partical.velocity.vy -= GRAVITY;

    let dest_x = x + partical.velocity.vx as isize;
    let dest_y = y + partical.velocity.vy as isize;

    line(x, y, dest_x, dest_y)
}

fn simulate_sand(grid: &mut map::Map, x: isize, y: isize) {
    let is_occupied_down = !grid.is_avalible(x, y - 1);
    let is_occupied_down_left = !grid.is_avalible(x - 1, y - 1);
    let is_occupied_down_right = !grid.is_avalible(x + 1, y - 1);

    // dbg!(is_grounded, is_occupied_left, is_occupied_right);

    let partical = grid.get_mut_at(x, y).unwrap();

    if is_occupied_down {
        partical.velocity.vy = 0.0;

        match (is_occupied_down_left, is_occupied_down_right) {
            (true, false) => {
                partical.velocity.vx += 1.0 * rand::thread_rng().gen_range(1..=3) as f32;
            }
            (false, true) => {
                partical.velocity.vx += -1.0 * rand::thread_rng().gen_range(1..=3) as f32;
            }
            (false, false) => {
                if rand::thread_rng().gen_bool(0.5) {
                    partical.velocity.vx += 1.0;
                } else {
                    partical.velocity.vx += -1.0;
                }
            }
            (true, true) => return,
        };
    }

    for window in apply_force(grid, x, y).windows(2) {
        let (x1, y1) = (window[0].0, window[0].1);
        let (x2, y2) = (window[1].0, window[1].1);

        if let Some(p1) = grid.get_at(x1, y1) {
            if p1.partical_type != ParticalType::Sand {
                continue;
            }
        } else {
            continue;
        }

        if let Some(p2) = grid.get_at(x2, y2) {
            if p2.partical_type != ParticalType::Air {
                continue;
            }
        } else {
            continue;
        }

        if grid.swap_checked(x1, y1, x2, y2).is_some() {
            // successfull swap and continue to next swap test
            continue;
        } else {
            return;
        };
    }
}

fn simulate_water(grid: &mut map::Map, x: isize, y: isize) {
    let is_occupied_down = !grid.is_avalible(x, y - 1);
    let is_occupied_down_left = !grid.is_avalible(x - 1, y - 1);
    let is_occupied_down_right = !grid.is_avalible(x + 1, y - 1);

    let is_occupied_left = !grid.is_avalible(x - 1, y);
    let is_occupied_right = !grid.is_avalible(x + 1, y);

    // dbg!(is_grounded, is_occupied_left, is_occupied_right);

    let partical = grid.get_mut_at(x, y).unwrap();

    if is_occupied_down {
        partical.velocity.vy = 0.0;

        match (
            is_occupied_left,
            is_occupied_down_left,
            is_occupied_down_right,
            is_occupied_right,
        ) {
            (false, true, true, false) => {
                if partical.velocity.vx == 0.0 {

                    if rand::thread_rng().gen_bool(0.5) {
                        partical.velocity.vx += 1.0;
                    } else {
                        partical.velocity.vx += -1.0;
                    }
                }  
            }

            (false, true, true, true) => {
                partical.velocity.vx += -1.0 * rand::thread_rng().gen_range(1..=2) as f32;
            }
            (true, true, true, false) => {
                partical.velocity.vx += 1.0 * rand::thread_rng().gen_range(1..=2) as f32;
            }

            (_, false, false, _) => {
                if rand::thread_rng().gen_bool(0.5) {
                    partical.velocity.vx += 1.0;
                } else {
                    partical.velocity.vx += -1.0;
                }
            }
            (_, true, false, _) => {
                partical.velocity.vx = 1.0 * rand::thread_rng().gen_range(1..=2) as f32;
            }
            (_, false, true, _) => {
                partical.velocity.vx = -1.0 * rand::thread_rng().gen_range(1..=2) as f32;
            }


            (true, true, true, true) => return,
        };
    }

    for window in apply_force(grid, x, y).windows(2) {
        let (x1, y1) = (window[0].0, window[0].1);
        let (x2, y2) = (window[1].0, window[1].1);

        if let Some(p1) = grid.get_at(x1, y1) {
            if p1.partical_type != ParticalType::Water {
                continue;
            }
        } else {
            continue;
        }

        if let Some(p2) = grid.get_at(x2, y2) {
            if p2.partical_type != ParticalType::Air {
                continue;
            }
        } else {
            continue;
        }

        if grid.swap_checked(x1, y1, x2, y2).is_some() {
            // successfull swap and continue to next swap test
            continue;
        } else {
            return;
        };
    }
}

fn simulate_smoke(_grid: &mut map::Map, _x: isize, _y: isize) {}

fn simulate_fire(_grid: &mut map::Map, _x: isize, _y: isize) {
    let mut gen = rand::thread_rng();
    if gen.gen_bool(0.01) {}
}

pub fn get_neighbors(width: isize, height: isize, include_center: bool) -> Vec<(isize, isize)> {
    let size = height * width - include_center as isize;
    let mut neighbors: Vec<(isize, isize)> = Vec::with_capacity(size as usize);

    for x in -width..=width {
        for y in -height..=height {
            if x == y {
                continue;
            }
            neighbors.push((x, y));
        }
    }
    neighbors
}

pub fn update(grid: &mut map::Map) {
    // grid.grid.iter().rev().flatten()
    for y in 0..map::GRID_HEIGHT {
        for x in 0..map::GRID_WIDTH {
            if let Some(partical) = grid.get_mut_at(x as isize, y as isize) {
                if partical.is_updated {
                    continue;
                }

                partical.is_updated = true;

                match partical.partical_type {
                    map::ParticalType::Air => {}
                    map::ParticalType::Sand => simulate_sand(grid, x as isize, y as isize),
                    map::ParticalType::Rock => {}
                    map::ParticalType::Water => simulate_water(grid, x as isize, y as isize),
                    map::ParticalType::Wood => {}
                    map::ParticalType::Fire => simulate_fire(grid, x as isize, y as isize),
                    map::ParticalType::Smoke => simulate_smoke(grid, x as isize, y as isize),
                };
            }
        }
    }

    for p in grid.grid.iter_mut().flatten() {
        p.is_updated = false;
    }
}

// fn bi_directional_range(range: std::ops::Range<isize>) -> impl Iterator<Item = isize> {

//     let step = if range.start > range.end { -1 } else { 1 };
//     (0..=(range.end - range.start).abs() as usize)
//         .map(move |i| range.start + (i as isize) * step)
// }

trait BiDirectionalRange {
    fn bi_directional_range(self) -> Box<dyn Iterator<Item = isize>>;
}

impl BiDirectionalRange for std::ops::Range<isize> {
    fn bi_directional_range(self) -> Box<dyn Iterator<Item = isize>> {
        let step = if self.start > self.end { -1 } else { 1 };
        Box::new(
            (0..=(self.end - self.start).unsigned_abs())
                .map(move |i| self.start + (i as isize) * step),
        )
    }
}

#[test]
fn bi_directional_range_test() {
    let test: Vec<isize> = (10..5).bi_directional_range().collect();
    let exp = [10, 9, 8, 7, 6, 5];
    assert_eq!(test, exp);

    let test: Vec<isize> = (10..5).collect();
    let exp = [];
    assert_eq!(test, exp);
}

#[test]
fn line_test_vertical_down() {
    let test = line(0, 1, 0, -5);
    let exp = vec![(0, 1), (0, 0), (0, -1), (0, -2), (0, -3), (0, -4), (0, -5)];
    assert_eq!(test, exp);
}
#[test]
fn line_test_vertical_up() {
    let test = line(0, -1, 0, 3);
    let exp = vec![(0, -1), (0, 0), (0, 1), (0, 2), (0, 3)];
    assert_eq!(test, exp);
}
#[test]
fn line_test_right_up() {
    let test = line(0, -1, 2, 3);
    let exp = vec![(0, -1), (0, 0), (1, 1), (1, 2), (2, 3)];
    assert_eq!(test, exp);
}

#[test]
fn line_test_left_down() {
    let test = line(0, -1, -5, -3);
    let exp = vec![(0, -1), (-1, -1), (-2, -2), (-3, -2), (-4, -3), (-5, -3)];
    assert_eq!(test, exp);
}
