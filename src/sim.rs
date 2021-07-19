use std::slice::GroupBy;

use rand::{prelude::SliceRandom, Rng};

use crate::grid::{self, Change, Partical, ParticalType};

fn simulate_sand(grid: &mut grid::Grid, x: isize, y: isize) {
    // If there is a point directly down
    // we want to move there first.
    let mut success;
    success = grid.checked_swap_cell(x, y, x, y + 1);

    // The reason we split the probablity to go left or right
    // is to create a more uniform distrobution of particals
    if !success {
        let mut gen = rand::thread_rng();

        if gen.gen_bool(0.5) {
            grid.checked_swap_cell(x, y, x + 1, y + 1);
            // if !success {
            //     success = grid.checked_swap_cell(x, y, x - 1, y + 1);
            // }
        } else {
            grid.checked_swap_cell(x, y, x - 1, y + 1);
            // if !success {
            //     success = grid.checked_swap_cell(x, y, x + 1, y + 1);
            // }
        }
    }
}

fn simulate_water(grid: &mut grid::Grid, x: isize, y: isize) {
    // If there is a point directly down
    // we want to move there first.

    let mut success;
    success = grid.checked_swap_cell(x, y, x, y + 1);

    // The reason we split the probablity to go left or right
    // is to create a more uniform distrobution of particals
    if !success {
        let mut gen = rand::thread_rng();

        // If there is a point to the left or right
        // we want to move there last
        if gen.gen_bool(0.5) {
            grid.checked_swap_cell(x, y, x + 1, y);
            //grid.checked_swap_cell(x, y, x - 1, y);
        } else {
            grid.checked_swap_cell(x, y, x - 1, y);
            //grid.checked_swap_cell(x, y, x + 1, y);
        }
    }
}

fn simulate_smoke(grid: &mut grid::Grid, x: isize, y: isize) {
    // If there is a point directly up
    // we want to move there first.

    let mut success;
    success = grid.checked_swap_cell(x, y, x, y - 1);

    // The reason we split the probablity to go left or right
    // is to create a more uniform distrobution of particals
    if !success {
        let mut gen = rand::thread_rng();

        // If there is a point to the left or right
        // we want to move there last
        if gen.gen_bool(0.5) {
            grid.checked_swap_cell(x, y, x + 1, y);
            //grid.checked_swap_cell(x, y, x - 1, y);
        } else {
            grid.checked_swap_cell(x, y, x - 1, y);
            //grid.checked_swap_cell(x, y, x + 1, y);
        }
    }
}

fn simulate_fire(grid: &mut grid::Grid, x: isize, y: isize) {
    let mut gen = rand::thread_rng();

    // spread fire
    get_neighbors(1, 1, false).iter().for_each(|(xx, yy)| {
        if gen.gen_bool(0.01) {
            if grid
                .get_at(x + xx, y + yy)
                .unwrap_or(&Partical::default())
                .partical_type
                == grid::ParticalType::Wood
            {
                let p = Partical {
                    partical_type: grid::ParticalType::Fire,
                };

                grid.checked_set_cell(p, x + xx, y + yy);
            }
        }
    });

    // Randomly change a fire partical to smoke
    if gen.gen_bool(0.01) {
        // 25% chance to set the fire to smoke
        if gen.gen_bool(0.25) {
            let p = Partical {
                partical_type: grid::ParticalType::Smoke,
            };
            grid.checked_set_cell(p, x, y);

        // 75% chance to set the fire to smoke
        } else {
            let p = Partical {
                partical_type: grid::ParticalType::Air,
            };
            grid.checked_set_cell(p, x, y);
        }
    }
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

pub fn simulate_points(grid: &mut grid::Grid) {
    for y in (0..grid::GRID_HEIGHT).rev() {
        for x in 0..grid::GRID_WIDTH {
            let p: &mut grid::Partical = grid.get_mut_at(x as isize, y as isize).unwrap();

            match p.partical_type {
                grid::ParticalType::Air => {}
                grid::ParticalType::Sand => simulate_sand(grid, x as isize, y as isize),
                grid::ParticalType::Rock => {}
                grid::ParticalType::Water => simulate_water(grid, x as isize, y as isize),
                grid::ParticalType::Wood => {}
                grid::ParticalType::Fire => simulate_fire(grid, x as isize, y as isize),
                grid::ParticalType::Smoke => simulate_smoke(grid, x as isize, y as isize),
            }
        }
    }
}

pub fn update(grid: &mut grid::Grid) {
    let mut rng = rand::thread_rng();

    // Sort the list of changes so that the destination is ordered
    grid.sort_by_destination();

    // Group all 'same' destinations in to a list
    // group_by is a Nightly feature
    let groups = grid.cell_changes.group_by(|a, b| {
        // When comparing, we need to take just the destination
        // from the `change` enum. Then compare by that feild.
        let c = match a {
            Change::Swap(_, _, x, y) => grid::get_index(*x, *y),
            Change::Set(_, x, y) => grid::get_index(*x, *y),
        };
        let d = match b {
            Change::Swap(_, _, x, y) => grid::get_index(*x, *y),
            Change::Set(_, x, y) => grid::get_index(*x, *y),
        };

        c == d
    });

    // Then pick a random destination from each list
    // and create a new list. Now, we should have only
    // one change for each cell in the grid.
    let changes: Vec<grid::Change> = groups.map(|g| *g.choose(&mut rng).unwrap()).collect();

    // Apply the changes to the grid.
    for change in changes {
        match change {
            Change::Swap(x1, y1, x2, y2) => grid.swap(x1, y1, x2, y2).unwrap(),
            Change::Set(p, x, y) => grid.set_at(x, y, p).unwrap(),
        }
    }

    // Remove changes from cell_changes vec
    grid.cell_changes.clear();
}
