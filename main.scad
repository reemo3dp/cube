include<data.scad>;

$fn = 100;

PUZZLE_SIZE = 20*3;

SIZE=PUZZLE_SIZE/DIM;
FUDGE=4;
HOLE_DIAM = 6;
NUB_DIAM = HOLE_DIAM+6;
NUB_DEPTH = SIZE/8;
PIN_DIAM = NUB_DIAM;
PIN_FUDGE = 0.01;

PARTIAL=false;
FIRST_OR_SECOND=1;



in = [-1, 0, 0];
out = [0, 1, 0];
whole_piece(in, out);
translate([40, 0, 0]) whole_piece(in, out*-1);

translate([80, 0, 0]) whole_piece(out, in);

module whole_piece(in, out) {
    rotate_piece(in, out) 
        piece(in, out);
}


module puzzle(path) {
    puzzle_rec(undef, path[0], tail(path));
}

module puzzle_rec(in, current, rest) {
    out = len(rest) > 0 ? rest[0] - current : undef;
    
    translate(current*SIZE) 
        scale(1-FUDGE/SIZE) 
            whole_piece(in, out);
    
    if(len(rest) > 0) {
        puzzle_rec(out*-1, rest[0], tail(rest));
    }
}



module piece(in, out) {
    if(in == undef && out != undef) {
        start_piece();
    } else if(in != undef && out == undef) {
        end_piece();
    } else if(in*-1 == out) {
        straight_piece();
    } else {
        curved_piece();
    }
}



module rotate_piece(in, out) {
    if(in*-1 == out) {
        rotate([(out[2]+out[1]*2)*90, 0, out[0]*-90]) children();
    } else {
        rotate([90*(1+out[1]), 0, 0]) rotate([0, 0, in[0]*90]) children();
    }
}


function tail(xs) = len(xs) > 1 ? [ for (i = [1:len(xs)-1]) xs[i] ] : [];

// Pieces

module base_cube() {
    cube(SIZE, center = true);
}

module male_connector() {
    difference() {
        cylinder(h = NUB_DEPTH, d = NUB_DIAM, center = true);
        cylinder(h = NUB_DEPTH, d = HOLE_DIAM, center = true);
    }
}

module curved_piece() {
    difference() {
        union() {
            base_cube();
            rotate([0, 0, -90]) translate([0, SIZE/2+NUB_DEPTH/2, 0]) rotate([90, 0, 0]) scale([0.95, 0.95, 1]) male_connector();
        };
        rotate([0, 0, 90]) translate([-SIZE/2,-SIZE/2,0]) 
            rotate_extrude(angle=90, $fn = 100)
                translate([SIZE/2, 0]) circle(d = HOLE_DIAM, $fn = 100);

        translate([0, -SIZE/2+NUB_DEPTH/2, 0]) rotate([90, 0, 0]) cylinder(h = NUB_DEPTH, d = NUB_DIAM, center = true);
    }
}

module straight_piece() {
    difference() {
        union() {
            base_cube();
            translate([0, SIZE/2+NUB_DEPTH/2, 0]) rotate([90, 0, 0]) scale([0.95, 0.95, 1]) male_connector();
        };
        rotate([90, 0, 0]) cylinder(d = HOLE_DIAM, h = SIZE, center = true);
        translate([0, -SIZE/2+NUB_DEPTH/2, 0]) rotate([90, 0, 0]) male_connector();
    }
}

module start_piece() {
    difference() {
        union() {
            base_cube();
            translate([0, SIZE/2+NUB_DEPTH/2, 0]) rotate([90, 0, 0]) scale([0.95, 0.95, 1]) male_connector();
            scale([1, 1, 1]*(1-PIN_FUDGE)) pin();
        };
        translate([0, SIZE/4, 0]) rotate([90, 0, 0]) cylinder(d = HOLE_DIAM, h = SIZE/2, center = true);
        pin();
    }
}

module end_piece() {
    difference() {
        union() {
            base_cube();
            scale([1, 1, 1]*(1-PIN_FUDGE)) pin();
        };
        translate([0, -SIZE/4, 0]) rotate([90, 0, 0]) cylinder(d = HOLE_DIAM, h = SIZE/2, center = true);
        pin();
        translate([0, -SIZE/2+NUB_DEPTH/2, 0]) rotate([90, 0, 0]) male_connector();
    }
}


module pin() {
    translate([SIZE*0.1, 0, 0]) rotate([0, 90, 0]) cylinder(h = SIZE*0.8, d = PIN_DIAM, center = true);
}

