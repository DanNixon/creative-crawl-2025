include <dimensions.scad>

module Plate() {
  difference() {
    // Body
    square([width, depth], center=true);

    // Mounting holes
    d = (width - side_block_size) / 2;
    for(x = [-d, d]) {
      for(y = [-plate_mounting_screw_centres/ 2, plate_mounting_screw_centres/ 2]) {
        translate([x, y]) {
          circle(d = plate_mounting_screw_size + 0.3, $fn = 16);
        }
      }
    }

    // Switches
    for(x = [-80, -40, 0, 40, 80]) {
      translate([x, 12]) {
        // Suspension wire hold
        circle(d = 4);

        // Switch block mounting holes
        translate([-6, -15]) {
          for(y = [-switch_mounting_screw_centres, 0]) {
            translate([0, y]) {
              circle(d = switch_mounting_screw_size + 0.3, $fn = 16);
            }
          }
        }
      }
    }
  }
}

Plate();
