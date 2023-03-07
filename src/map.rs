use rand::Rng;
use sdl2::pixels::Color;
// use crate::map::_IMPL_NUM_FromPrimitive_FOR_ParticalType::_num_traits::Saturating;

pub const WINDOW_WIDTH: usize = 1000 * 2;
pub const WINDOW_HEIGHT: usize = 500 * 2;

const A: usize = 1;

pub const GRID_WIDTH: usize = 500 * A;
pub const GRID_HEIGHT: usize = 250 * A;

#[derive(Debug, Clone, PartialEq, PartialOrd, FromPrimitive, Default)]
pub enum ParticalType {
    #[default]
    Air,
    Sand,
    Rock,
    Water,
    Wood,
    Fire,
    Smoke,
}

#[derive(Default, Debug, Clone)]
pub struct Velocity {
    pub vx: f32,
    pub vy: f32,
}

#[derive(Debug, Default, Clone)]
pub struct Partical {
    pub partical_type: ParticalType,
    // pub lifetime
    pub velocity: Velocity,
    pub is_updated: bool,
    pub color: Vec<u8>,
}

impl Partical {
    pub fn new(p_type: ParticalType) -> Partical {
        Partical {
            is_updated: false,
            color: color_lookup(&p_type),
            partical_type: p_type,
            velocity: Velocity::default(),
        }
    }
}

fn color_lookup(p_type: &ParticalType) -> Vec<u8> {
    match p_type {
        ParticalType::Air => vec![255, 255, 255]
            .into_iter()
            .map(|value: u8| value.saturating_add_signed(rand::thread_rng().gen_range(-10..1i8)))
            .collect::<Vec<u8>>(),

        ParticalType::Sand => vec![192, 178, 128]
            .into_iter()
            .map(|value: u8| value.saturating_add_signed(rand::thread_rng().gen_range(-10i8..10)))
            .collect::<Vec<u8>>(),

        ParticalType::Rock => vec![135, 135, 135]
            .into_iter()
            .map(|value: u8| value.saturating_add_signed(rand::thread_rng().gen_range(-10i8..10)))
            .collect::<Vec<u8>>(),

        ParticalType::Water => vec![0, 50, 255]
            .into_iter()
            .map(|value: u8| value.saturating_add_signed(rand::thread_rng().gen_range(-10i8..10)))
            .collect::<Vec<u8>>(),

        ParticalType::Wood => vec![54, 38, 27]
            .into_iter()
            .map(|value: u8| value.saturating_add_signed(rand::thread_rng().gen_range(-10i8..10)))
            .collect::<Vec<u8>>(),

        ParticalType::Fire => vec![226, 88, 34]
            .into_iter()
            .map(|value: u8| value.saturating_add_signed(rand::thread_rng().gen_range(-10i8..10)))
            .collect::<Vec<u8>>(),

        ParticalType::Smoke => vec![115, 130, 118]
            .into_iter()
            .map(|value: u8| value.saturating_add_signed(rand::thread_rng().gen_range(-10i8..10)))
            .collect::<Vec<u8>>(),
    }
}

// type Grid = [[u32; GRID_WIDTH]; GRID_HEIGHT];

#[derive(Debug)]
pub struct Map {
    pub grid: Vec<Vec<Partical>>,
}

pub fn get_index(x: isize, y: isize) -> isize {
    x + y * GRID_WIDTH as isize
}

impl Map {
    pub fn get_at(&self, x: isize, y: isize) -> Option<&Partical> {
        self.grid.get(y as usize)?.get(x as usize)
    }

    pub fn get_mut_at(&mut self, x: isize, y: isize) -> Option<&mut Partical> {
        self.grid.get_mut(y as usize)?.get_mut(x as usize)
    }

    pub fn set_at(&mut self, x: isize, y: isize, partical: Partical) -> Option<()> {
        if !self.within_bounds(x, y) {
            return None;
        }

        self.grid[y as usize][x as usize] = partical;
        Some(())
    }

    pub fn get_coord(&self, index: isize) -> (isize, isize) {
        let x = index.rem_euclid(GRID_WIDTH as isize);
        let y = index / GRID_WIDTH as isize;

        (x, y)
    }

    pub fn within_bounds(&self, x: isize, y: isize) -> bool {
        y >= 0 && y < GRID_HEIGHT as isize && x >= 0 && x < GRID_WIDTH as isize
    }

    pub fn swap(&mut self, x1: isize, y1: isize, x2: isize, y2: isize) -> Option<()> {
        let p2 = self.get_mut_at(x2, y2).unwrap().clone();
        let p1 = self.get_mut_at(x1, y1).unwrap().clone();

        self.set_at(x2, y2, p1).unwrap();
        self.set_at(x1, y1, p2).unwrap();

        Some(())
    }

    pub fn swap_checked(&mut self, x1: isize, y1: isize, x2: isize, y2: isize) -> Option<()> {
        if self.within_bounds(x2, y2) {
            return self.swap(x1, y1, x2, y2);
        }

        None
    }

    pub fn is_empty(&self, x: isize, y: isize) -> bool {
        if !self.within_bounds(x, y) {
            return false;
        }

        if self.get_at(x, y).is_some() {
            return false;
        }

        true
    }
    pub fn is_avalible(&self, x: isize, y: isize) -> bool {
        if !self.within_bounds(x, y) {
            return false;
        }

        if let Some(val) = self.get_at(x, y) {
            return val.partical_type == ParticalType::Air;
        }

        false
    }

    pub fn is_occupied(&self, x: isize, y: isize) -> bool {
        if !self.within_bounds(x, y) {
            return false;
        }

        if self.get_at(x, y).is_some() {
            return true;
        }

        false
    }
}

#[test]
fn test_saturating_add_on_unsigned() {
    let a = 0u8.saturating_add_signed(-10i8);
    let b = 255u8.saturating_add_signed(10i8);

    assert_eq!(a, 0);
    assert_eq!(b, 255);
}
