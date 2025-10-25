include <dimensions.scad>

side_mounting_screw_diameter = 4.5;
side_mounting_screw_centres = 60;

module SideBlock() {
  difference() {
    // Body
    cube([side_block_size, depth, side_block_size], center=true);

    // Top/plate mounting holes
    for(y = [-plate_mounting_screw_centres/ 2, plate_mounting_screw_centres/ 2]) {
      translate([0, y, 0]) {
        cylinder(h = side_block_size + 0.1, d = plate_mounting_screw_size, center = true, $fn = 5);
      }
    }

    // Side/box mounting holes
    for(y = [-side_mounting_screw_centres / 2, side_mounting_screw_centres / 2]) {
      translate([0, y, 0]) {
        rotate([0, 90, 0]) {
          cylinder(h = side_block_size + 0.1, d = side_mounting_screw_diameter, center = true, $fn = 16);
        }
      }
    }
  }
}

SideBlock();
