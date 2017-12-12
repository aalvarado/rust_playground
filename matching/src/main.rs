const COORDS: [(isize,isize); 8] = [
    (-1, -1),
    (-1,  0),
    (-1,  1),
    ( 0, -1),
    ( 0,  1),
    ( 1, -1),
    ( 1,  0),
    ( 1,  1)
];

#[derive(Clone)]
pub struct Cell {
    status: u8
}

impl Cell {
    pub fn new() -> Cell {
        Cell { status: 0 }
    }
}


pub struct Grid {
    cells: Vec<Cell>
}

fn main() {
    let width = 10;
    let grid = Grid { cells: vec![Cell::new(); width] };
    let cell = cell_at(&grid, 4);

    match cell {
        Some(cell) => println!("value {}", cell.status),
        _ => ()
    }
}

pub fn cell_at(grid: &Grid, x: usize) -> Option<& Cell> {
    if x <= grid.cells.len() {
        Some(&grid.cells[x])
    } else {
        None
    }
}
