use crate::Solve;

#[derive(Debug)]
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

enum Direction {
    Left,
    Up,
    Right,
    Down,
    Front,
    Back
}

struct Position {
    pos: (i8, i8, i8),
    direction: Direction
}

impl Position {

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
}
