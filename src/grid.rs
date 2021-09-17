use rand::prelude::SliceRandom;

// GRID width and height ratio should be
// the same as Window width and height

pub const WINDOW_WIDTH: usize = 1000;
pub const WINDOW_HEIGHT: usize = 500;

pub const GRID_WIDTH: usize = 500;
pub const GRID_HEIGHT: usize = 250;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, FromPrimitive)]
pub enum ParticalType {
    Air,
    Sand,
    Rock,
    Water,
    Wood,
    Fire,
    Smoke,
}

impl Default for ParticalType {
    fn default() -> ParticalType {
        ParticalType::Air
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Change {
    Swap(isize, isize, isize, isize), // Coords    X1, Y1, X2, Y2
    Set(Partical, isize, isize),      // Partical, X1, Y1
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Partical {
    pub partical_type: ParticalType,
}

impl Default for Partical {
    fn default() -> Partical {
        Partical {
            partical_type: ParticalType::default(),
        }
    }
}

impl Partical {
    pub fn new() -> Partical {
        Partical {
            partical_type: ParticalType::default(),
        }
    }
}

#[derive(Debug)]
pub struct Grid {
    pub grid: Vec<Vec<Partical>>,
    pub cell_changes: Vec<Change>, // Index1, Index2
}

pub fn get_index(x: isize, y: isize) -> isize {
    x + y * GRID_WIDTH as isize
}

impl Grid {
    pub fn get_at_index(&self, index: isize) -> Option<&Partical> {
        self.grid.iter().flatten().nth(index as usize)
    }

    pub fn get_at(&self, x: isize, y: isize) -> Option<&Partical> {
        self.grid.get(y as usize)?.get(x as usize)
    }

    pub fn get_mut_at(&mut self, x: isize, y: isize) -> Option<&mut Partical> {
        self.grid.get_mut(y as usize)?.get_mut(x as usize)
    }

    pub fn set_at(&mut self, x: isize, y: isize, partical: Partical) -> Option<()> {
        //self.grid[y][x] = partical; // Unsafe
        *self.grid.get_mut(y as usize)?.get_mut(x as usize)? = partical;

        Some(())
    }
    // This will give the coords of a given index.
    // This will not garrenty that the coords are within the bounds
    // of the grid.
    pub fn get_coord(&self, index: isize) -> (isize, isize) {
        let x = index.rem_euclid(GRID_WIDTH as isize);
        let y = index / GRID_WIDTH as isize;

        (x, y)
    }

    // Simple check to verify that the position within
    // the grid exists.
    // Returns true if value are a valid location.
    pub fn within_bounds(&self, x: isize, y: isize) -> bool {
        if y >= 0 && y < GRID_HEIGHT as isize && x >= 0 && x < GRID_WIDTH as isize {
            return true;
        }
        false
    }

    pub fn sort_by_destination(&mut self) {
        self.cell_changes
            .sort_unstable_by(|a: &Change, b: &Change| {
                // When comparing, we need to take just the destination
                // from the `change` enum. Then compare by that feild.
                match a {
                    Change::Swap(_, _, x, y) => get_index(*x, *y),
                    Change::Set(_, x, y) => get_index(*x, *y),
                }
                .cmp(&match b {
                    Change::Swap(_, _, x, y) => get_index(*x, *y),
                    Change::Set(_, x, y) => get_index(*x, *y),
                })
            })
    }

    // This function swaps particals given two locations.
    // Returned is a Option, If the position doesnt exist, it crashes.
    // Does this needlessly Clone?
    pub fn swap(&mut self, x1: isize, y1: isize, x2: isize, y2: isize) -> Option<()> {
        let p1 = *self.get_at(x1, y1)?;
        let p2 = *self.get_at(x2, y2)?;
        self.set_at(x2, y2, p1)?;
        self.set_at(x1, y1, p2)?;

        Some(())
    }

    // Checks if the points to swap are with in bounds and
    // if the cell is air.
    // Returns `true` on a successfull change of the cell.
    // Returns `false` on a non successfull change.
    pub fn checked_swap_cell(&mut self, x1: isize, y1: isize, x2: isize, y2: isize) -> bool {
        if self.within_bounds(x2, y2) {
            let p2 = self.get_at(x2, y2).unwrap();

            if p2.partical_type == ParticalType::Air {
                let change = Change::Swap(x1, y1, x2, y2);

                self.cell_changes.push(change);
                return true;
            }
        }

        false
    }

    pub fn checked_set_cell(&mut self, partical: Partical, x2: isize, y2: isize) {
        if self.within_bounds(x2, y2) {
            let change = Change::Set(partical, x2, y2);

            self.cell_changes.push(change);
        }
    }
}
