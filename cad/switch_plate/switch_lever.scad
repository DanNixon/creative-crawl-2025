include <dimensions.scad>

length = 30;
height = 4;
thickness = 3;

module SwitchLever() {
  difference() {
    linear_extrude(height) {
      difference() {
        // Body
        union() {
          circle(d = switch_arm_mounting_screw_size + 4, $fn = 32);
          translate([length / 2, 0]) {
            square([length, thickness], center = true);
          }
        }

        // Block mounting hole
        circle(d = switch_arm_mounting_screw_size + 0.3, $fn = 16);
      }
    }

    // Wire tie point
    translate([length - 2, 0, height / 2]) {
      rotate([90, 0, 0]) {
        cylinder(d = 1.5, h = thickness + 0.1, center = true, $fn = 16);
      }
    }
  }
}

SwitchLever();
