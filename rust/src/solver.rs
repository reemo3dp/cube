use std::{
    collections::HashSet,
    fmt::UpperExp,
    ops::{Add, Not, Sub},
    thread::current,
};

use indexmap::{set::Difference, IndexSet};

use crate::Solve;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Piece {
    Straight,
    Curve,
}

enum Instruction {
    Straight,
    Left,
    Up,
    Right,
    Down,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
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
    fn project(&self, other: Position) -> Position {
        match(self) {
            Direction::Left => todo!(),
            Direction::Up => todo!(),
            Direction::Right => todo!(),
            Direction::Down => todo!(),
            Direction::Forward => todo!(),
            Direction::Backward => todo!(),
        }
    }

    fn project(&self, other: Direction) -> Direction {
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

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct PieceInPosition {
    pos: Position,
    direction: Direction,
    piece: Piece,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Position(i8, i8, i8);
impl Position {
    fn min(self, other: Position) -> Position {
        Position(
            std::cmp::min(self.0, other.0),
            std::cmp::min(self.1, other.1),
            std::cmp::min(self.2, other.2),
        )
    }
    fn max(self, other: Position) -> Position {
        Position(
            std::cmp::max(self.0, other.0),
            std::cmp::max(self.1, other.1),
            std::cmp::max(self.2, other.2),
        )
    }
}

impl Sub<Position> for Position {
    type Output = Position;

    fn sub(self, rhs: Position) -> Self::Output {
        Position(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Add<(i8, i8, i8)> for Position {
    type Output = Position;

    fn add(self, rhs: (i8, i8, i8)) -> Self::Output {
        Position(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl PieceInPosition {
    fn all_next_positions(&self, piece: Piece) -> HashSet<PieceInPosition> {
        if piece == Piece::Straight {
            return HashSet::from([PieceInPosition {
                direction: self.direction,
                pos: self.pos + (1, 0, 0),
                piece,
            }]);
        }

        let mut result = HashSet::new();
        for n in [
            Direction::Left,
            Direction::Up,
            Direction::Right,
            Direction::Down,
        ] {
            result.insert(PieceInPosition {
                direction: self.direction.project(n),
                pos: self.pos,
                piece,
            });
        }
        result
    }
}

pub fn solve(solve: Solve) {
    let pieces: Vec<_> = solve
        .chain
        .chars()
        .map(|c| match c {
            'C' => Piece::Curve,
            'S' => Piece::Straight,
            c => panic!("No such character: {}", c),
        })
        .collect();

    for d in [
        Direction::Backward,
        Direction::Down,
        Direction::Forward,
        Direction::Left,
        Direction::Right,
        Direction::Up,
    ] {
        let mut chain = IndexSet::new();
        chain.insert(PieceInPosition {
            piece: Piece::Straight,
            pos: Position(0, 0, 0),
            direction: d,
        });

        solve_rec(chain, pieces.clone(), Position(0, 0, 0), Position(0, 0, 0));
    }
}

fn solve_rec(
    pieces_in_position: IndexSet<PieceInPosition>,
    rest: Vec<Piece>,
    current_min: Position,
    current_max: Position,
) -> Option<IndexSet<PieceInPosition>> {
    let spread = rest.split_first();
    if spread.is_none() {
        let dim = (pieces_in_position.len() as f32).powf(1.0 / 3.0) as i8;

        if current_max - current_min == Position(dim, dim, dim) {
            return Some(pieces_in_position);
        }
        return None;
    }
    let (next, new_rest) = spread.unwrap();

    let binding = pieces_in_position.last().unwrap().all_next_positions(*next);

    let all_next_positions = binding.iter().filter(|x| !pieces_in_position.contains(*x));

    for next_position in all_next_positions {
        let mut next_pieces = pieces_in_position.clone();
        if !next_pieces.insert(*next_position) {
            continue;
        }

        let result = solve_rec(
            next_pieces,
            new_rest.to_vec(),
            current_min.min(next_position.pos),
            current_max.max(next_position.pos),
        );
        if result.is_some() {
            return result;
        }
    }

    None
}
