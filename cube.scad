include<data_3_easier.scad>;


PUZZLE_SIZE = 25*3;

SIZE=PUZZLE_SIZE/DIM;
FUDGE=4;
HOLE_DIAM = 5;
NUB_DIAM = HOLE_DIAM+3;
CHAMFER_WIDTH = 2;

PARTIAL=false;
FIRST_OR_SECOND=1;


module curve(from, to) {
    both = from+to;
    if(from[2] == 0 && to[2] == 0) {
        // Both are level
        if(both == [-1, 1, 0]) {
            mirror([0, 1, 0]) pipe();
        } else if(both == [1, 1, 0]) {
            mirror([1, 1, 0]) pipe();
        } else if(both == [-1, -1, 0]) {
            mirror([0, 0, 0]) pipe();
        } else if(both == [1, -1, 0]) {
            mirror([1, 0, 0]) pipe();
        } else {
            throw("error");
        }
    } else {
        if(both == [-1, 0, -1]) {
            mirror([0, 0, 1]) rotate([90, 90, 0]) pipe();
        } else if(both == [0, 1, -1]) {
            mirror([0, 0, 1]) rotate([180, 90, 0]) pipe();
        } else if(both == [1, 0, 1]) {
            mirror([0, 0, 0]) rotate([270, 90, 0]) pipe();
        } else if(both == [-1, 0, 1]) {
            mirror([0, 0, 0]) rotate([90, 90, 0]) pipe();
        } else if(both == [0, -1, -1]) {
           mirror([0, 0, 1]) rotate([0, 90, 0]) pipe();
        } else if(both == [0, 1, 1]) {
           mirror([0, 0, 0]) rotate([180, 90, 0]) pipe();
        } else if(both == [1, 0, -1]) {
           mirror([0, 0, 1]) rotate([270, 90, 0]) pipe();
        } else if(both == [0, -1, 1]) {
           mirror([0, 0, 0]) rotate([0, 90, 0]) pipe();
        } else {
           throw("WOOP");
        }
    }
}

module pipe() {
    translate([-SIZE/2,-SIZE/2,0]) rotate_extrude(angle=90, $fn = 100)
            translate([SIZE/2, 0]) circle(d = HOLE_DIAM, $fn = 100);
}

module piece(prevDirection, direction) {
    rot = !direction ? [0,0,0] :
            direction[2] == -1 ? [180, 0, 0 ] :
                [-direction[1], direction[0], 0]*90; 

    rotIn = !prevDirection ? [0,0,0] :
            prevDirection[2] == -1 ? [180, 0, 0 ] :
                [-prevDirection[1], prevDirection[0], 0]*90; 
    difference() {
        union() {
            cube(SIZE, center = true);
            
            if(direction) {
                rotate(rot) translate([0, 0, SIZE/2]) difference() {
                    cylinder(h = SIZE/4, d = NUB_DIAM, center = true, $fn = 100);
                    cylinder(h = SIZE/4, d = HOLE_DIAM, center = true, $fn = 100);
                }
            }
        }
        union() {
            if(prevDirection) {
                rotate(rotIn) translate([0, 0, SIZE/2]) cylinder(h = SIZE/4, d = NUB_DIAM*1.1, center = true, $fn = 100);
                if(direction) {
                    if(direction+prevDirection == [0, 0, 0]) {
                        rotate(rotIn) translate([0, 0, SIZE/3]) cylinder(h = SIZE*2, d = HOLE_DIAM, center = true, $fn = 100);
                    } else {
                        curve(prevDirection, direction);
                    }
                }
            }
            
            if(!prevDirection) {
               rotate(rot+[90*direction[2], 90*direction[0], 90*(direction[2]+3*direction[1])]) 
                   translate([0, 0, SIZE*0.1]) cylinder(h = SIZE*0.9, d = NUB_DIAM, center = true, $fn = 100);
               
               rotate(rot) translate([0, 0, SIZE/4]) cylinder(h = SIZE/2, d = HOLE_DIAM, center = true, $fn = 100);
                
            }
            if(!direction) {
               rotate(rotIn+[90*prevDirection[2], 90*prevDirection[0], 90*(prevDirection[2]+3*prevDirection[1])]) 
                   translate([0, 0, SIZE*0.1]) cylinder(h = SIZE*0.9, d = NUB_DIAM, center = true, $fn = 100);
               
               rotate(rotIn) translate([0, 0, SIZE/4]) cylinder(h = SIZE/2, d = HOLE_DIAM, center = true, $fn = 100);
            
            }
            
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
    if(!prevDirection) {
        rotate(rot+[90*direction[2], 90*direction[0], 90*(direction[2]+3*direction[1])]) 
           translate([0, 0, SIZE*0.1]) cylinder(h = SIZE*0.8, d = NUB_DIAM*0.9, center = true, $fn = 100);
    }
    if(!direction) {
        rotate(rotIn+[90*prevDirection[2], 90*prevDirection[0], 90*(prevDirection[2]+3*prevDirection[1])]) 
           translate([0, 0, SIZE*0.1]) cylinder(h = SIZE*0.8, d = NUB_DIAM*0.9, center = true, $fn = 100);
    }
    
}




function tail(xs) = len(xs) > 1 ? [ for (i = [1:len(xs)-1]) xs[i] ] : [];

module puzzle(path) {
    puzzle_piece(undef, path[0], tail(path));
}

module puzzle_piece(prevDirection, curr, rest) {
    let(direction = len(rest)>0 ? rest[0]-curr : undef, index = DIM*DIM*DIM-len(rest)) {
        echo(index, index %2 == 1 ? "A" : "B",  !prevDirection ? "START" : !direction ? "END" : prevDirection*-1 == direction ? "STRAIGHT" : "CURVE");
        if(!PARTIAL || len(rest) % 2 == FIRST_OR_SECOND) {
            color(alpha=0.8,c = [0.5,0,0]+[0.5, 0, 0]*index/(DIM*DIM*DIM)) translate(curr*SIZE) 
                scale([1, 1, 1]*(1-FUDGE/SIZE))  
                    piece(prevDirection, direction);

        }
        if(len(rest) > 0) {
            puzzle_piece(direction*-1, rest[0], tail(rest));
        }
    }
}

puzzle(PATH);
//piece([-1, 0, 0], undef);