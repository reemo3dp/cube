use core::panic;
use std::ops::{Add, Not, Sub};

use indexmap::IndexSet;

use crate::Solve;

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
            Direction::Left => Vector3d(-1, 0, 0),
            Direction::Up => Vector3d(0, 0, 1),
            Direction::Right => Vector3d(1, 0, 0),
            Direction::Down => Vector3d(0, 0, -1),
            Direction::Forward => Vector3d(0, 1, 0),
            Direction::Backward => Vector3d(0, -1, 0),
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

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Vector3d(i8, i8, i8);
impl Vector3d {
    fn min(self, other: Vector3d) -> Vector3d {
        Vector3d(
            std::cmp::min(self.0, other.0),
            std::cmp::min(self.1, other.1),
            std::cmp::min(self.2, other.2),
        )
    }
    fn max(self, other: Vector3d) -> Vector3d {
        Vector3d(
            std::cmp::max(self.0, other.0),
            std::cmp::max(self.1, other.1),
            std::cmp::max(self.2, other.2),
        )
    }
}

impl Sub<Vector3d> for Vector3d {
    type Output = Vector3d;

    fn sub(self, rhs: Vector3d) -> Self::Output {
        Vector3d(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Add<(i8, i8, i8)> for Vector3d {
    type Output = Vector3d;

    fn add(self, rhs: (i8, i8, i8)) -> Self::Output {
        Vector3d(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Add<Vector3d> for Vector3d {
    type Output = Vector3d;

    fn add(self, rhs: Vector3d) -> Self::Output {
        Vector3d(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl PieceInPosition {
    fn all_next_positions(&self, piece: Piece) -> Vec<PieceInPosition> {
        if piece == Piece::Straight {
            return Vec::from([PieceInPosition {
                facing_direction: self.facing_direction,
                position: self.position + self.facing_direction.normal(),
                piece,
            }]);
        }

        let mut result = Vec::new();
        for n in [
            Direction::Left,
            Direction::Up,
            Direction::Right,
            Direction::Down,
        ] {
            result.push(PieceInPosition {
                facing_direction: self.facing_direction.project_direction(n),
                position: self.position + self.facing_direction.normal(),
                piece,
            });
        }
        result
    }
}

fn print_chain(c: IndexSet<PieceInPosition>) {
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

    let mut chain = IndexSet::new();
    chain.insert(PieceInPosition {
        piece: Piece::Start,
        position: Vector3d(0, 0, 0),
        facing_direction: Direction::Forward,
    });

    if let Some(solution) = solve_rec(
        chain,
        pieces.clone()[1..].to_vec(),
        Vector3d(0, 0, 0),
        Vector3d(0, 0, 0),
        dim,
    ) {
        print_chain(solution);
        return;
    }
}

fn solve_rec(
    pieces_in_position: IndexSet<PieceInPosition>,
    rest: Vec<Piece>,
    current_min: Vector3d,
    current_max: Vector3d,
    dim: i8,
) -> Option<IndexSet<PieceInPosition>> {
    let spread = rest.split_first();

    if spread.is_none() {
        if (current_max - current_min) == Vector3d(dim, dim, dim) {
            return Some(pieces_in_position);
        }
        return Some(pieces_in_position);
        //return None;
    }
    let (next, new_rest) = spread.unwrap();

    let binding = pieces_in_position.last().unwrap().all_next_positions(*next);

    let all_next_positions = binding
        .iter()
        .filter(|x| !pieces_in_position.iter().any(|p| p.position == x.position));

    for next_position in all_next_positions {
        let mut next_pieces = pieces_in_position.clone();
        if !next_pieces.insert(*next_position) {
            continue;
        }

        let next_min = current_min.min(next_position.position);
        let next_max = current_max.max(next_position.position);
        let diff = next_max - next_min;
        if diff.0 >= dim || diff.1 >= dim || diff.2 >= dim {
            continue;
        }

        let result = solve_rec(next_pieces, new_rest.to_vec(), next_min, next_max, dim);
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
            position: Vector3d(0, 0, 0),
            facing_direction: Direction::Up,
            piece: Piece::Straight,
        };

        assert_eq!(
            Vec::from_iter(piece.all_next_positions(Piece::Straight).iter()),
            Vec::from_iter(
                [PieceInPosition {
                    position: Vector3d(0, 0, 1),
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
            position: Vector3d(0, 0, 0),
            facing_direction: Direction::Up,
            piece: Piece::Straight,
        };

        assert_eq!(
            piece.all_next_positions(Piece::Curve),
            Vec::from([
                PieceInPosition {
                    position: Vector3d(0, 0, 1),
                    facing_direction: Direction::Left,
                    piece: Piece::Curve
                },
                PieceInPosition {
                    position: Vector3d(0, 0, 1),
                    facing_direction: Direction::Backward,
                    piece: Piece::Curve
                },
                PieceInPosition {
                    position: Vector3d(0, 0, 1),
                    facing_direction: Direction::Right,
                    piece: Piece::Curve
                },
                PieceInPosition {
                    position: Vector3d(0, 0, 1),
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
