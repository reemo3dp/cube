use crate::{NUM_TRIED, PRINT_EVERY};

pub type Coord = [u8; 3];
pub const VALID_NEIGHBOURS: [[i8; 3]; 6] = [
    [-1, 0, 0],
    [1, 0, 0],
    [0, -1, 0],
    [0, 1, 0],
    [0, 0, -1],
    [0, 0, 1],
];

pub fn record_failure(chain_len: usize) {
    unsafe {
        NUM_TRIED += 1;
        if PRINT_EVERY > 0 && NUM_TRIED % PRINT_EVERY == 0 {
            eprintln!("//D Stopping at {}", chain_len);
            eprintln!(
                "//D {}",
                NUM_TRIED as f64 / (crate::STARTED.elapsed().as_millis() as f64)
            );
        }
    }
}

pub struct Cube {
    pub dim: u8,
    pub path: Vec<Coord>,
}

pub fn get_neighbours(coord: Coord, dim: u8) -> Vec<Coord> {
    let mut neighbours = Vec::with_capacity(6); // Pre-allocate for max possible neighbours
    let [x, y, z] = coord;

    // Since we know x, y, z are u8, we can do cheaper bounds checking
    for &[dx, dy, dz] in VALID_NEIGHBOURS.iter() {
        // Handle negative results first
        if (dx < 0 && x == 0) || (dy < 0 && y == 0) || (dz < 0 && z == 0) {
            continue;
        }

        // Safe unsigned arithmetic for positive offsets
        let nx = if dx < 0 {
            x - (-dx as u8)
        } else {
            x + (dx as u8)
        };
        let ny = if dy < 0 {
            y - (-dy as u8)
        } else {
            y + (dy as u8)
        };
        let nz = if dz < 0 {
            z - (-dz as u8)
        } else {
            z + (dz as u8)
        };

        // Check upper bounds
        if nx < dim && ny < dim && nz < dim {
            neighbours.push([nx, ny, nz]);
        }
    }

    neighbours
}
