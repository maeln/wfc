extern crate rand;

use rand::prelude::*;

use std::vec::Vec;

/*
0 -> Nada 🚫
1 -> sea 🌊
2 -> sand 🏖
3 -> Soil 🌲

Rules:
- Sea can only be next to sand and sea
- Sand can be next to sea and earth
- Soil can only be next to sand and earth
*/

const WIDTH: usize = 32;
const HEIGHT: usize = 19;

const NADA: usize = 0;
const SEA: usize = 1;
const SAND: usize = 2;
const SOIL: usize = 3;

enum WFC_STATE {
    FINISHED,
    CONTRADICTION,
    COLLAPSED,
}

#[derive(Clone, PartialEq)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    pub fn new(x: usize, y: usize) -> Coord {
        Coord { x, y }
    }
}

#[derive(Clone, PartialEq)]
struct Matrix {
    width: usize,
    height: usize,
    data: Vec<usize>,
}

impl Matrix {
    pub fn new(width: usize, height: usize, default_value: usize) -> Matrix {
        Matrix {
            width,
            height,
            data: vec![default_value; width * height],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> usize {
        self.data[y * self.width + x]
    }

    pub fn set(&mut self, x: usize, y: usize, v: usize) {
        self.data[y * self.width + x] = v;
    }

    pub fn neighbors(&self, x: usize, y: usize) -> Vec<Coord> {
        let mut neighbors: Vec<Coord> = Vec::new();

        if x > 0 {
            neighbors.push(Coord::new(x - 1, y));
        }
        if x < (WIDTH - 1) {
            neighbors.push(Coord::new(x + 1, y));
        }
        if y > 0 {
            neighbors.push(Coord::new(x, y - 1));
        }
        if y < (HEIGHT - 1) {
            neighbors.push(Coord::new(x, y + 1));
        }
        neighbors
    }

    pub fn neighbors_value(&self, x: usize, y: usize) -> Vec<usize> {
        let mut neighbors: Vec<Coord> = self.neighbors(x, y);
        neighbors.into_iter().map(|n| self.get(n.x, n.y)).collect()
    }

    fn print_matrix(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.get(x, y) {
                    SEA => print!("🌊"),
                    SAND => print!("🏖"),
                    SOIL => print!("🌲"),
                    _ => print!("🚫"),
                };
            }
            print!("\n");
        }
    }
}

fn possibilities(matrix: &Matrix, x: usize, y: usize) -> Vec<usize> {
    if matrix.get(x, y) != NADA {
        return vec![];
    }
    let mut pos: Vec<usize> = Vec::new();
    // No condition for sand
    pos.push(SAND);

    // Sea: no soil neighbors
    if !matrix.neighbors_value(x, y).contains(&SOIL) {
        pos.push(SEA);
    }

    // Soil: no sea neighbors
    if !matrix.neighbors_value(x, y).contains(&SEA) {
        pos.push(SOIL);
    }

    pos
}

/// Return false if it could not collapse.
fn collapse(matrix: &mut Matrix, x: usize, y: usize) -> bool {
    let pos = possibilities(matrix, x, y);
    let mut rng = thread_rng();
    if pos.is_empty() {
        return false;
    }

    matrix.set(x, y, pos.choose(&mut rng).unwrap().clone());
    true
}

fn is_finished(matrix: &Matrix) -> bool {
    !matrix.data.contains(&NADA)
}

fn pos_neighbors(matrix: &Matrix, x: usize, y: usize) -> (usize, usize) {
    let mut nb_pos = 4;
    let mut neighbor = (0, 0);
    if x > 0 {
        let pos = possibilities(matrix, x - 1, y);
        if !pos.is_empty() && pos.len() < nb_pos {
            neighbor = (x - 1, y);
            nb_pos = pos.len();
        }
    }

    if x < (WIDTH - 1) {
        let pos = possibilities(matrix, x + 1, y);
        if !pos.is_empty() && pos.len() < nb_pos {
            neighbor = (x + 1, y);
            nb_pos = pos.len();
        }
    }

    if y > 0 {
        let pos = possibilities(matrix, x, y - 1);
        if !pos.is_empty() && pos.len() < nb_pos {
            neighbor = (x, y - 1);
            nb_pos = pos.len();
        }
    }

    if y < (HEIGHT - 1) {
        let pos = possibilities(matrix, x, y + 1);
        if !pos.is_empty() && pos.len() < nb_pos {
            neighbor = (x, y + 1);
            nb_pos = pos.len();
        }
    }

    neighbor
}

fn wfc(matrix: &mut Matrix, x: usize, y: usize) -> WFC_STATE {
    // collapse itself:
    let collapsed = collapse(matrix, x, y);
    if !collapsed {
        return WFC_STATE::CONTRADICTION;
    }

    if is_finished(matrix) {
        return WFC_STATE::FINISHED;
    }

    // We find the neighbors with the least choise:
    let neighbor = pos_neighbors(matrix, x, y);
    wfc(matrix, neighbor.0, neighbor.1)
}

fn main() {
    let mut matrix: Matrix = Matrix::new(WIDTH, HEIGHT, 0);
    let rt = wfc(&mut matrix, 0, 0);
    match rt {
        WFC_STATE::CONTRADICTION => println!("CONTRADICTION"),
        WFC_STATE::COLLAPSED => println!("COLLAPSED"),
        WFC_STATE::FINISHED => println!("FINISHED"),
    }
    matrix.print_matrix();
}
