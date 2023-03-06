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
}

impl Partical {
    pub fn new() -> Partical {
        Partical {
            is_updated: false,
            partical_type: ParticalType::default(),
            velocity: Velocity::default(),
        }
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
