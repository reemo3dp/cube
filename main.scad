$fn = 100;

PUZZLE_SIZE = 20*3;

SIZE=PUZZLE_SIZE/DIM;
EXPLODE=10;
HOLE_DIAM = 5;
NUB_DIAM = HOLE_DIAM+4;
NUB_DEPTH = SIZE/5;
NUB_FUDGE_FACTOR = 1.01;
PIN_DIAM = NUB_DIAM;
PIN_FUDGE = 0.01;
CHAMFER_WIDTH = 1;

CONNECTOR_EDGE = 0;
CONNECTOR_GAP = 0;

COLORS = ["purple", "black"];

puzzle(PATH, 0);
puzzle(PATH, 1);

module whole_piece(in, out) {
    piece(in, out);
}

function to_str(d) =
    d[0] == -1 ? "LEFT" :
        d[0] == 1 ? "RIGHT" :
            d[1] == -1 ? "FRONT" :
                d[1] == 1 ? "BACK" :
                    d[2] == -1 ? "DOWN" :
                        d[2] == 1 ? "UP" : never();


                        
module puzzle(path, only) {
    puzzle_rec(undef, path[0], tail(path), only);
}

module puzzle_rec(in, current, rest, only) {
    out = len(rest) > 0 ? rest[0] - current : undef;
    index = DIM*DIM*DIM-len(rest);

    if(index % 2 == only) {
        color(alpha=0.95,c = COLORS[index % 2]) translate(current*SIZE*(1+EXPLODE/SIZE)) 
            whole_piece(in, out);
    }
    
    if(len(rest) > 0) {
        puzzle_rec(out*-1, rest[0], tail(rest), only);
    }
}

function never() = assert(false);


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
            rotate([180, -90, 180]) curved_piece();
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
            curved_piece();
        }
    }
}

function tail(xs) = len(xs) > 1 ? [ for (i = [1:len(xs)-1]) xs[i] ] : [];

module base_cube() {
    difference() {
        cube(SIZE, center = true);
        
        translate([-SIZE, -SIZE, 0]/2) rotate([0, 0, 45]) cube([CHAMFER_WIDTH, CHAMFER_WIDTH, SIZE], center = true);
mirror([1, 0, 0]) translate([-SIZE, -SIZE, 0]/2) rotate([0, 0, 45]) cube([CHAMFER_WIDTH, CHAMFER_WIDTH, SIZE], center = true);
mirror([1, 1, 0]) translate([-SIZE, -SIZE, 0]/2) rotate([0, 0, 45]) cube([CHAMFER_WIDTH, CHAMFER_WIDTH, SIZE], center = true);
mirror([0, 1, 0]) translate([-SIZE, -SIZE, 0]/2) rotate([0, 0, 45]) cube([CHAMFER_WIDTH, CHAMFER_WIDTH, SIZE], center = true);
rotate([90, 0, 0]) union() {
    translate([-SIZE, -SIZE, 0]/2) rotate([0, 0, 45]) cube([CHAMFER_WIDTH, CHAMFER_WIDTH, SIZE], center = true);
    mirror([1, 0, 0]) translate([-SIZE, -SIZE, 0]/2) rotate([0, 0, 45]) cube([CHAMFER_WIDTH, CHAMFER_WIDTH, SIZE], center = true);
    mirror([1, 1, 0]) translate([-SIZE, -SIZE, 0]/2) rotate([0, 0, 45]) cube([CHAMFER_WIDTH, CHAMFER_WIDTH, SIZE], center = true);
    mirror([0, 1, 0]) translate([-SIZE, -SIZE, 0]/2) rotate([0, 0, 45]) cube([CHAMFER_WIDTH, CHAMFER_WIDTH, SIZE], center = true);
}
rotate([0, 90, 0]) union() {
    translate([-SIZE, -SIZE, 0]/2) rotate([0, 0, 45]) cube([CHAMFER_WIDTH, CHAMFER_WIDTH, SIZE], center = true);
    mirror([1, 0, 0]) translate([-SIZE, -SIZE, 0]/2) rotate([0, 0, 45]) cube([CHAMFER_WIDTH, CHAMFER_WIDTH, SIZE], center = true);
    mirror([1, 1, 0]) translate([-SIZE, -SIZE, 0]/2) rotate([0, 0, 45]) cube([CHAMFER_WIDTH, CHAMFER_WIDTH, SIZE], center = true);
    mirror([0, 1, 0]) translate([-SIZE, -SIZE, 0]/2) rotate([0, 0, 45]) cube([CHAMFER_WIDTH, CHAMFER_WIDTH, SIZE], center = true);
            }
    }
}

module male_connector() {
    difference() {
        union() {
            cylinder(h = NUB_DEPTH, d2 = NUB_DIAM, d1 = NUB_DIAM+CONNECTOR_EDGE, center = true);
            rotate([0, 0, 45]) translate([0, 0, (NUB_DEPTH)/2-0.5]) { 
                cube([SIZE*0.8, 1.2, 1], center = true);
                cube([1.2, SIZE*0.8, 1], center = true);
            }            
        }
        cylinder(h = NUB_DEPTH, d = HOLE_DIAM, center = true);
        cube([CONNECTOR_GAP, NUB_DIAM+CONNECTOR_EDGE, NUB_DEPTH], center= true);
        cube([NUB_DIAM+CONNECTOR_EDGE, CONNECTOR_GAP, NUB_DEPTH], center= true);
    }
}

module female_connector() {
    cylinder(h = NUB_DEPTH, d2 = NUB_FUDGE_FACTOR*NUB_DIAM, d1 = NUB_FUDGE_FACTOR*(NUB_DIAM+CONNECTOR_EDGE), center = true);
    rotate([0, 0, 45]) translate([0, 0, (NUB_DEPTH)/2-0.6]) { 
        cube([SIZE*0.8, 1.8, 1.4], center = true);
        cube([1.8, SIZE*0.8, 1.4], center = true);
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

        translate([0, -SIZE/2+NUB_DEPTH/2, 0]) rotate([90, 0, 0]) female_connector();
    }
}

module straight_piece() {
    difference() {
        union() {
            base_cube();
            translate([0, SIZE/2+NUB_DEPTH/2, 0]) rotate([90, 0, 0]) scale([0.95, 0.95, 1]) male_connector();
        };
        rotate([90, 0, 0]) cylinder(d = HOLE_DIAM, h = SIZE, center = true);
        translate([0, -SIZE/2+NUB_DEPTH/2, 0]) rotate([90, 0, 0]) female_connector();
    }
}

module start_piece() {
    difference() {
        union() {
            base_cube();
            translate([0, SIZE/2+NUB_DEPTH/2, 0]) rotate([90, 0, 0]) scale([0.95, 0.95, 1]) male_connector();
        };
        pin_cutout();
        translate([0, SIZE/2, 0]) rotate([90, 0, 0]) cylinder(d = HOLE_DIAM, h = SIZE, center = true);
    }
    scale([1, 1, 1]*(1-PIN_FUDGE)) pin();
}   

module end_piece() {
    difference() {
        base_cube();
        translate([0, -SIZE/4, 0]) rotate([90, 0, 0]) cylinder(d = HOLE_DIAM, h = SIZE/2, center = true);
        pin_cutout();
        translate([0, -SIZE/2+NUB_DEPTH/2, 0]) rotate([90, 0, 0]) female_connector();
    }
    scale([1, 1, 1]*(1-PIN_FUDGE)) pin();    
}


module pin_cutout() {
    translate([SIZE*0.1, 0, 0]) rotate([0, 90, 0]) cylinder(h = SIZE*0.8, d = PIN_DIAM, center = true);
}

module pin() {
    translate([SIZE*0.1, 0, 0]) rotate([0, 90, 0]) 
        difference() {
            union() {
                difference() {
                    cylinder(h = SIZE*0.8, d = PIN_DIAM, center = true);
                    cylinder(h = SIZE*0.8/2, d = PIN_DIAM, center = true);
                }
                cylinder(h = SIZE*0.8/2, d1 = PIN_DIAM, d2 = PIN_DIAM*0.5, center = true);
                cylinder(h = SIZE*0.8/2, d2 = PIN_DIAM, d1 = PIN_DIAM*0.5, center = true);
            }
            translate([0, PIN_DIAM/2, 0]) cube([SIZE*0.8, SIZE*0.1, SIZE*0.8], center= true);
    }
}

