include<data_3_easier.scad>;

$fn = 100;

PUZZLE_SIZE = 20*5;

SIZE=PUZZLE_SIZE/DIM;
FUDGE=4;
HOLE_DIAM = 6;
NUB_DIAM = HOLE_DIAM+6;
NUB_DEPTH = SIZE/8;
PIN_DIAM = NUB_DIAM;
PIN_FUDGE = 0.01;

PARTIAL=false;
FIRST_OR_SECOND=1;



//whole_piece([0, 0, -1], [0, 0, 1]);
//translate([30, 0, 0]) whole_piece([0, 0, -1]*-1, [0, 0, 1]*-1);
//translate([60, 0, 0]) whole_piece([-1, 0, 0], [1, 0, 0]);;
//translate([90, 0, 0]) whole_piece([-1, 0, 0]*-1, [1, 0, 0]*-1);


//DOWN-> BACK
//translate([ 0, 30, 0]) whole_piece([0, 0, -1], [0, 1, 0]);
//translate([ 30, 30, 0]) whole_piece([0, 1, 0], [0, 0, -1]);

//UP -> FRONT
//translate([0, 30, 0]) whole_piece([0, 0, -1]*-1, [0, 1, 0]*-1);
//translate([30, 30, 0]) whole_piece([0, 1, 0]*-1, [0, 0, -1]*-1);

//RIGHT -> DOWN
//translate([0, 30, 0]) whole_piece([1, 0, 0], [0, 0, -1]);
//translate([30, 30, 0]) whole_piece([0, 0, -1], [1, 0, 0]);

////LEFT-> UP
//translate([0, 30, 0]) whole_piece([1, 0, 0]*-1, [0, 0, -1]*-1);
//translate([30, 30, 0]) whole_piece([0, 0, -1]*-1, [1, 0, 0]*-1);

//LEFT-> DOWN
//translate([0, 30, 0]) whole_piece([-1, 0, 0], [0, 0, -1]);
//translate([30, 30, 0]) whole_piece([0, 0, -1], [-1, 0, 0]);

//translate([ 0, 30, 0]) whole_piece([-1, 0, 0], [1, 0, 0]);
//translate([30, 30, 0]) whole_piece([1, 0, 0], [-1, 0, 0]);
//translate([60, 30, 0]) whole_piece([0, -1, 0], [0, 1, 0]);
//translate([90, 30, 0]) whole_piece([0, 1, 0], [0, -1, 0]);
//translate([120, 30, 0]) whole_piece([0, 0, -1], [0, 0, 1]);
//translate([150, 30, 0]) whole_piece([0, 0,  1], [0, 0, -1]);


module whole_piece(in, out) {
    piece(in, out);
}

function to_str(d) =
    d[0] == -1 ? "LEFT" :
        d[0] == 1 ? "RIGHT" :
            d[1] == -1 ? "FRONT" :
                d[1] == 1 ? "BACK" :
                    d[2] == -1 ? "DOWN" :
                        d[2] == 1 ? "UP" : "ILLEGAL";


puzzle(PATH);

                        
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


function combination_matches(xs, ys) = 
    (xs[0] == ys[0] && xs[1] == ys[1]) ||
        (xs[0] == ys[1] && xs[1] == ys[0]);


module piece(in, out) {
    if(in == undef && out != undef) {
        rotate([90*out[2], 0, out[1] == -1 ? 180 : out[0]*-90]) start_piece();
    } else if(in != undef && out == undef) {
        rotate([-90*in[2], 0, in[1] == 1 ? 180 : in[0]*90]) end_piece();
    } else if(in*-1 == out) {
        rotate([-90*abs(in[2]), 0, -90*abs(in[0])]) 
            mirror([0, (in[0]+in[1]+in[2]) > 0 ? 1 : 0, 0]) straight_piece();
    } else {
        inStr = to_str(in);
        outStr = to_str(out);
        if(inStr == "DOWN" && outStr == "BACK") {
            rotate([90, 0, 90]) curved_piece();
        } else if(inStr == "DOWN" && outStr == "RIGHT") {
            rotate([90, 0, 0]) curved_piece();
        } else if(inStr == "DOWN" && outStr == "LEFT") {
            rotate([90, 0, 180]) curved_piece();
        } else if(inStr == "DOWN" && outStr == "FRONT") {
            rotate([90, 0, -90]) curved_piece();
        } else if(inStr == "BACK" && outStr == "DOWN") {
            rotate([180,90,0]) curved_piece();
        } else if(inStr == "BACK" && outStr == "RIGHT") {
            rotate([180, 0, 0]) curved_piece();
        } else if(inStr == "BACK" && outStr == "LEFT") {
            rotate([180, 180, 0]) curved_piece();
        } else if(inStr == "BACK" && outStr == "UP") {
            rotate([0, -90, 180]) curved_piece();
        } else if(inStr == "UP" && outStr == "LEFT") {
            rotate([-90, 0, 180]) curved_piece();
        } else if(inStr == "UP" && outStr == "RIGHT") {
            rotate([-90, 0, 0]) curved_piece();
        } else if(inStr == "UP" && outStr == "FRONT") {
            rotate([-90,0,-90]) curved_piece();
        } else if(inStr == "UP" && outStr == "BACK") {
            rotate([-90, 0, 90]) curved_piece();
        } else if(inStr == "FRONT" && outStr == "RIGHT") {
            rotate([0, 0, 0]) curved_piece();
        } else if(inStr == "FRONT" && outStr == "UP") {
            rotate([180, -90, 0]) curved_piece();
        } else if(inStr == "FRONT" && outStr == "DOWN") {
            rotate([0, 90, 0]) curved_piece();
        } else if(inStr == "FRONT" && outStr == "LEFT") {
            rotate([0, 180, 0]) curved_piece();
        } else if(inStr == "LEFT" && outStr == "UP") {
            rotate([0, -90, -90]) curved_piece();
        } else if(inStr == "LEFT" && outStr == "DOWN") {
            rotate([0, 90, -90]) curved_piece();
        } else if(inStr == "LEFT" && outStr == "BACK") {
            rotate([0, 180, -90]) curved_piece();
        } else if(inStr == "LEFT" && outStr == "FRONT") {
            rotate([0, 0, -90]) curved_piece();
        } else if(inStr == "RIGHT" && outStr == "DOWN") {
            rotate([0, 90, 90]) curved_piece();
        } else if(inStr == "RIGHT" && outStr == "UP") {
            rotate([0, -90, 90]) curved_piece();
        } else if(inStr == "RIGHT" && outStr == "FRONT") {
            rotate([0, 180, 90]) curved_piece();
        } else if(inStr == "RIGHT" && outStr == "BACK") {
            rotate([0, 0, 90]) curved_piece();
        } else {
            echo(in, to_str(in), "to", out, to_str(out));
            #curved_piece();
        }
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

