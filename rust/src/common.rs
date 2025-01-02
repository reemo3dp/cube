use crate::{NUM_SOLUTIONS_TRIED, PRINT_DEBUG_EVERY_N_FAILURES, SHOULD_STOP};

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
    let num_tried = NUM_SOLUTIONS_TRIED.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let print_every = PRINT_DEBUG_EVERY_N_FAILURES.load(std::sync::atomic::Ordering::Relaxed);
    if !SHOULD_STOP.load(std::sync::atomic::Ordering::Relaxed)
        && print_every > 0
        && num_tried % print_every == 0
    {
        eprint!("\r");
        eprint!(
            "//D Last chain: {:3}, {:10.3} chains/ms for {:6}s (tried {:6} chains)",
            chain_len,
            num_tried as f64 / (crate::STARTED_AT.elapsed().as_millis() as f64),
            crate::STARTED_AT.elapsed().as_secs(),
            format_number(num_tried)
        );
    }
}

const SUFFIXES: [&str; 6] = ["", "K", "M", "G", "T", "P"];

pub fn format_number(n: u64) -> String {
    for (i, suffix) in SUFFIXES.iter().rev().enumerate() {
        let pow: u64 = 1000_u64.pow((SUFFIXES.len() - i) as u32);
        if n >= pow {
            return format!("{}{}", n / pow, suffix);
        }
    }
    format!("{}", n)
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
