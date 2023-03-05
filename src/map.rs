pub const WINDOW_WIDTH: usize = 1000 * 2;
pub const WINDOW_HEIGHT: usize = 500 * 2;

pub const GRID_WIDTH: usize = 500;
pub const GRID_HEIGHT: usize = 250;

pub type Id = u16;

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

#[derive(Default, Debug)]
pub struct Velocity {
    pub vx: f32,
    pub vy: f32,
}

#[derive(Debug, Default)]
pub struct Partical {
    pub id: Id,
    pub partical_type: ParticalType,
    // pub lifetime
    pub velocity: Velocity,
    pub is_updated: bool,
}

impl Partical {
    pub fn new(id: Id) -> Partical {
        Partical {
            id,
            partical_type: ParticalType::default(),
            velocity: Velocity::default(),
            is_updated: false,
        }
    }
}

// type Grid = [[u32; GRID_WIDTH]; GRID_HEIGHT];

#[derive(Debug)]
pub struct Map {
    pub grid: [[Id; GRID_WIDTH]; GRID_HEIGHT],
    pub particals: Vec<Partical>,
    pub partical_index: Id,
}

pub fn get_index(x: isize, y: isize) -> isize {
    x + y * GRID_WIDTH as isize
}

impl Map {
    pub fn new_partical(&mut self) -> Id {
        self.partical_index += 1;
        self.particals.push(Partical::new(self.partical_index));

        self.partical_index
    }

    //     pub fn get_at_index(&self, index: isize) -> Option<&Partical> {
    //         self.grid.iter().flatten().nth(index as usize)
    //     }

    pub fn get_at(&self, x: isize, y: isize) -> Option<Id> {
        Some(*(self.grid.get(y as usize)?.get(x as usize)?))
    }

    pub fn set_at(&mut self, x: isize, y: isize, partical: Id) -> Option<()> {
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

    // pub fn get_partical_at_index(&self, index: isize) -> (isize, isize) {
    //     let x = index.rem_euclid(GRID_WIDTH as isize);
    //     let y = index / GRID_WIDTH as isize;

    //     (x, y)
    // }

    pub fn within_bounds(&self, x: isize, y: isize) -> bool {
        y >= 0 && y < GRID_HEIGHT as isize && x >= 0 && x < GRID_WIDTH as isize
    }

    pub fn get_partical(&self, id: Id) -> Option<&Partical> {
        self.particals.iter().find(|p| p.id == id)
    }
    pub fn get_partical_at(&self, x: isize, y: isize) -> Option<&Partical> {
        match self.get_at(x, y) {
            None => None,
            Some(id) => self.particals.iter().find(|p| p.id == id),
        }
    }

    pub fn get_mut_partical_at(&mut self, x: isize, y: isize) -> Option<&mut Partical> {
        match self.get_at(x, y) {
            None => None,
            Some(id) => self.particals.iter_mut().find(|p| p.id == id),
        }
    }

    pub fn get_mut_partical(&mut self, id: Id) -> Option<&mut Partical> {
        self.particals.iter_mut().find(|p| p.id == id)
    }

    pub fn swap(&mut self, x1: isize, y1: isize, x2: isize, y2: isize) -> Option<()> {
        let p1_id = self.get_at(x1, y1)?;
        let p2_id = self.get_at(x2, y2)?;

        self.set_at(x2, y2, p1_id)?;
        self.set_at(x1, y1, p2_id)?;

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

        let id = self.get_at(x, y).unwrap();
        if self.get_partical(id).is_some() {
            return false;
        }

        true
    }

    pub fn is_occupied(&self, x: isize, y: isize) -> bool {
        if !self.within_bounds(x, y) {
            return false;
        }

        if self.get_partical_at(x, y).is_some() {
            return true;
        }

        false
    }

    // pub fn checked_set(&mut self, partical: Partical, x2: isize, y2: isize) {
    //     if self.within_bounds(x2, y2) {
    //         let change = Change::Set(partical, x2, y2);

    //         self.cell_changes.push(change);
    //     }
    // }
}
