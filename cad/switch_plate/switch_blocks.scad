include <dimensions.scad>

size = [8, 22, 20];
switch_mounting_screw_diameter = 1.95;

module SwitchBlock() {
  difference() {
    // Body
    translate([0, 0, size[2] / 2]) {
      cube(size, center=true);
    }

    // Bottom/plate mounting holes
    for(y = [-switch_mounting_screw_centres / 2, switch_mounting_screw_centres / 2]) {
      translate([0, y, -0.05]) {
        cylinder(h = 10, d = switch_mounting_screw_size, $fn = 5);
      }
    }

    // Side/switch mounting holes
    for(p = [[10, 1], [10, 7]]) {
      translate([0, p[1], p[0]]) {
        rotate([0, 90, 0]) {
          cylinder(h = size[0] + 0.1, d = switch_mounting_screw_diameter, center = true, $fn = 5);
        }
      }
    }

    // Arm mounting hole
    translate([0, -8, 17]) {
      rotate([0, 90, 0]) {
        cylinder(h = size[0] + 0.1, d = switch_arm_mounting_screw_size, center = true, $fn = 5);
      }
    }
  }
}

SwitchBlock();
