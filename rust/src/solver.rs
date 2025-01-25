use std::{collections::HashSet, fmt::UpperExp, ops::{Add, Not}};

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
    Down
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
    Forward,
    Backward
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
    fn project(&self, other: Direction) -> Direction {
        match (*self, other){
            (Direction::Forward, other) => other,
            (current, Direction::Forward) => current,
            (_, Direction::Backward) => panic!("Can't go back"),
            
            (Direction::Left | Direction::Right, Direction::Up) => Direction::Up,
            (Direction::Left | Direction::Right, Direction::Down) => Direction::Down,
            (Direction::Left, Direction::Right) => Direction::Forward,
            (Direction::Left, Direction::Left) => Direction::Backward,

            (Direction::Backward, x) => !x,
            
            (Direction::Up | Direction::Down, x@ (Direction::Left | Direction::Right)) => x,

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
    piece: Piece
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Position(i8, i8, i8);

impl Add<(i8, i8, i8)> for Position {
    type Output = Position;

    fn add(self, rhs: (i8, i8, i8)) -> Self::Output {
        Position(
            self.0+rhs.0,
            self.1+rhs.1,
            self.2+rhs.2
        )
    }
}

impl PieceInPosition {
    fn allNextPositions(&self, piece: Piece) -> HashSet<PieceInPosition> {
        if piece == Piece::Straight {
            return HashSet::from([PieceInPosition {
                direction: self.direction,
                pos: self.pos + (1, 0, 0),
                piece,
            }]);
        }

        let mut result = HashSet::new();
        for n in [Direction::Left, Direction::Up, Direction::Right, Direction::Down] {
            result.insert(PieceInPosition {
                direction: self.direction.project(n),
                pos: self.pos,
                piece
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

    for d in [Direction::Backward, Direction::Down, Direction::Forward, Direction::Left, Direction::Right, Direction::Up] {
        let mut chain = IndexSet::new();
        chain.insert(PieceInPosition {
            piece: Piece::Straight,
            pos: Position(0, 0, 0),
            direction: d
        });

        solveRec(chain, pieces.clone());
    }
}

fn solveRec(piecesInPosition: IndexSet<PieceInPosition>, rest: Vec<Piece>) -> Option<IndexSet<PieceInPosition>> {
 
 
 None   
}
