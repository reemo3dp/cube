use ahash::{AHasher, RandomState};
use core::panic;
use std::{
    ops::Not,
    sync::{atomic::AtomicU64, Arc},
    time::Instant,
};

use crate::vector3d::Vector3d;

use indexmap::IndexSet;
use lazy_static::initialize;

use crate::Solve;

lazy_static! {
    pub static ref STARTED_AT: Instant = Instant::now();
    pub static ref NUM_SOLUTIONS_TRIED: Arc<AtomicU64> = Arc::new(AtomicU64::new(0));
    pub static ref PRINT_DEBUG_EVERY_N_FAILURES: Arc<AtomicU64> = Arc::new(AtomicU64::new(0));
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Piece {
    Straight,
    Curve,
    Start,
    End,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
    Forward,
    Backward,
}

pub fn record_failure(chain_len: usize) {
    let num_tried = NUM_SOLUTIONS_TRIED.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let print_every = PRINT_DEBUG_EVERY_N_FAILURES.load(std::sync::atomic::Ordering::Relaxed);
    if print_every > 0 && num_tried % print_every == 0 {
        eprint!("\r");
        eprint!(
            "//D Last chain: {:3}, {:10.3} chains/ms for {:6}s (tried {:6} chains)",
            chain_len,
            num_tried as f64 / (crate::generate::STARTED_AT.elapsed().as_millis() as f64),
            crate::generate::STARTED_AT.elapsed().as_secs(),
            num_tried
        );
    }
}

impl Not for Direction {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Direction::Left => Self::Right,
            Direction::Up => Self::Down,
            Direction::Right => Self::Left,
            Direction::Down => Self::Up,
            Direction::Forward => Self::Backward,
            Direction::Backward => Self::Forward,
        }
    }
}

impl Direction {
    fn normal(&self) -> Vector3d {
        match self {
            Direction::Left => Vector3d { x: -1, y: 0, z: 0 },
            Direction::Up => Vector3d { x: 0, y: 0, z: 1 },
            Direction::Right => Vector3d { x: 1, y: 0, z: 0 },
            Direction::Down => Vector3d { x: 0, y: 0, z: -1 },
            Direction::Forward => Vector3d { x: 0, y: 1, z: 0 },
            Direction::Backward => Vector3d { x: 0, y: -1, z: 0 },
        }
    }

    fn project_direction(&self, other: Direction) -> Direction {
        match (*self, other) {
            (Direction::Forward, other) => other,
            (current, Direction::Forward) => current,
            (_, Direction::Backward) => panic!("Can't go back"),

            (Direction::Left | Direction::Right, Direction::Up) => Direction::Up,
            (Direction::Left | Direction::Right, Direction::Down) => Direction::Down,
            (Direction::Left, Direction::Right) => Direction::Forward,
            (Direction::Left, Direction::Left) => Direction::Backward,

            (Direction::Backward, x) => !x,

            (Direction::Up | Direction::Down, x @ (Direction::Left | Direction::Right)) => x,

            (Direction::Up, Direction::Up) => Direction::Backward,
            (Direction::Up, Direction::Down) => Direction::Forward,

            (Direction::Right, Direction::Left) => Direction::Forward,
            (Direction::Right, Direction::Right) => Direction::Backward,

            (Direction::Down, Direction::Down) => Direction::Backward,
            (Direction::Down, Direction::Up) => Direction::Forward,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct PieceInPosition {
    position: Vector3d,
    facing_direction: Direction,
    piece: Piece,
}

impl PieceInPosition {
    fn all_next_positions(&self, piece: Piece) -> Vec<PieceInPosition> {
        if piece == Piece::Straight {
            return [PieceInPosition {
                facing_direction: self.facing_direction,
                position: self.position + self.facing_direction.normal(),
                piece,
            }]
            .to_vec();
        }

        [
            Direction::Left,
            Direction::Up,
            Direction::Right,
            Direction::Down,
        ]
        .map(|n| PieceInPosition {
            facing_direction: self.facing_direction.project_direction(n),
            position: self.position + self.facing_direction.normal(),
            piece,
        })
        .to_vec()
    }
}

fn print_chain(c: Vec<PieceInPosition>) {
    for (i, s) in c.iter().enumerate() {
        println!(
            "{}, Piece: {:?}, Position: {:?}, Direction: {:?}",
            i + 1,
            s.piece,
            s.position,
            s.facing_direction
        );
    }
}

pub fn solve(solve: Solve) {
    initialize(&STARTED_AT);
    if solve.verbose {
        PRINT_DEBUG_EVERY_N_FAILURES.store(1000000, std::sync::atomic::Ordering::Relaxed);
    }

    let mut pieces: Vec<_> = solve
        .chain
        .to_lowercase()
        .chars()
        .map(|c| match c {
            'c' => Piece::Curve,
            's' => Piece::Straight,
            c => panic!("No such character: {}", c),
        })
        .collect();

    pieces.insert(0, Piece::Start);
    pieces.push(Piece::End);

    let dim = (pieces.len() as f32).powf(1.0 / 3.0) as i8;

    if dim.pow(3) != pieces.len().try_into().unwrap() {
        panic!("Provided String does not form a cube: {}", pieces.len());
    }

    let mut chain = Vec::with_capacity((dim * dim * dim).try_into().unwrap());

    chain.push(PieceInPosition {
        piece: Piece::Start,
        position: Vector3d { x: 0, y: 0, z: 0 },
        facing_direction: Direction::Forward,
    });

    if let Some(solution) = solve_rec(
        chain,
        &pieces.clone()[1..],
        Vector3d { x: 0, y: 0, z: 0 },
        Vector3d { x: 0, y: 0, z: 0 },
        dim,
    ) {
        print_chain(solution);
    }
}

fn solve_rec(
    pieces_in_position: Vec<PieceInPosition>,
    rest: &[Piece],
    current_min: Vector3d,
    current_max: Vector3d,
    dim: i8,
) -> Option<Vec<PieceInPosition>> {
    if pieces_in_position.len() == (dim * dim * dim).try_into().unwrap() {
        if (current_max - current_min)
            == (Vector3d {
                x: dim - 1,
                y: dim - 1,
                z: dim - 1,
            })
        {
            return Some(pieces_in_position);
        }
        record_failure(pieces_in_position.len());
        return None;
    }

    let next = rest[0];

    let binding: Vec<PieceInPosition> = pieces_in_position.last().unwrap().all_next_positions(next);

    let all_next_positions = binding
        .iter()
        .filter(|x| !pieces_in_position.iter().any(|p| p.position == x.position));

    for next_position in all_next_positions {
        let next_min = current_min.min(next_position.position);
        let next_max = current_max.max(next_position.position);
        let diff = next_max - next_min;
        if diff.x >= dim || diff.y >= dim || diff.z >= dim {
            record_failure(pieces_in_position.len());
            continue;
        }

        let mut next_pieces = pieces_in_position.clone();
        next_pieces.push(*next_position);

        let result = solve_rec(next_pieces, &rest[1..], next_min, next_max, dim);
        if result.is_some() {
            return result;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_elements() {
        let piece = PieceInPosition {
            position: Vector3d { x: 0, y: 0, z: 0 },
            facing_direction: Direction::Up,
            piece: Piece::Straight,
        };

        assert_eq!(
            Vec::from_iter(piece.all_next_positions(Piece::Straight).iter()),
            Vec::from_iter(
                [PieceInPosition {
                    position: Vector3d { x: 0, y: 0, z: 1 },
                    facing_direction: Direction::Up,
                    piece: Piece::Straight
                }]
                .iter()
            )
        );
    }

    #[test]
    fn test_next_elements_curve() {
        let piece = PieceInPosition {
            position: Vector3d { x: 0, y: 0, z: 0 },
            facing_direction: Direction::Up,
            piece: Piece::Straight,
        };

        assert_eq!(
            piece.all_next_positions(Piece::Curve),
            Vec::from([
                PieceInPosition {
                    position: Vector3d { x: 0, y: 0, z: 1 },
                    facing_direction: Direction::Left,
                    piece: Piece::Curve
                },
                PieceInPosition {
                    position: Vector3d { x: 0, y: 0, z: 1 },
                    facing_direction: Direction::Backward,
                    piece: Piece::Curve
                },
                PieceInPosition {
                    position: Vector3d { x: 0, y: 0, z: 1 },
                    facing_direction: Direction::Right,
                    piece: Piece::Curve
                },
                PieceInPosition {
                    position: Vector3d { x: 0, y: 0, z: 1 },
                    facing_direction: Direction::Forward,
                    piece: Piece::Curve
                }
            ])
        );
    }

    #[test]
    fn test_projection_identity() {
        for direction in [
            Direction::Backward,
            Direction::Down,
            Direction::Forward,
            Direction::Left,
            Direction::Right,
            Direction::Up,
        ] {
            assert_eq!(Direction::Forward.project_direction(direction), direction);
            assert_eq!(direction.project_direction(Direction::Forward), direction);
        }
    }

    #[test]
    fn test_projection() {
        for (from, direction, result) in [
            (Direction::Left, Direction::Left, Direction::Backward),
            (Direction::Backward, Direction::Left, Direction::Right),
            (Direction::Right, Direction::Left, Direction::Forward),
            (Direction::Left, Direction::Right, Direction::Forward),
            (Direction::Backward, Direction::Right, Direction::Left),
            (Direction::Right, Direction::Right, Direction::Backward),
            (Direction::Up, Direction::Left, Direction::Left),
            (Direction::Up, Direction::Right, Direction::Right),
            (Direction::Down, Direction::Left, Direction::Left),
            (Direction::Down, Direction::Right, Direction::Right),
        ] {
            assert_eq!(
                from.project_direction(direction),
                result,
                "({:?}).projection_direction({:?}) != {:?}",
                from,
                direction,
                result
            );
        }
    }
}
