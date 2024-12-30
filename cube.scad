include<data_5.scad>;

SIZE=10;
FUDGE=2;

module piece(direction) {
    cube(SIZE-2*FUDGE, center = true);
}


function tail(xs) = len(xs) > 1 ? [ for (i = [1:len(xs)-1]) xs[i] ] : [];

module puzzle(path) {
    puzzle_piece(path[0], tail(path));
}

module puzzle_piece(curr, rest) {
    translate(curr*SIZE) union() { 
        color(c = [1, 0, 0]*(len(rest)/(DIM*DIM*DIM))) piece();
        if(len(rest) > 0) {
            direction = rest[0]-curr;
            
            translate(direction) sphere(d = SIZE/2, $fn = 100);
        }
    };
    if(len(rest) > 0) {
        puzzle_piece(rest[0], tail(rest));
    }
}

puzzle(PATH);



