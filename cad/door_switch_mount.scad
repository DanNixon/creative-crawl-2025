size = [4, 18, 20];
switch_mounting_screw_diameter = 1.95;

module DoorSwitchMount() {
  difference() {
    // Body
    translate([0, 0, size[2] / 2]) {
      cube(size, center=true);
    }

    // Side/switch mounting holes
    for(p = [[16, -3], [16, 3]]) {
      translate([0, p[1], p[0]]) {
        rotate([0, 90, 0]) {
          cylinder(h = size[0] + 0.1, d = switch_mounting_screw_diameter, center = true, $fn = 5);
        }
      }
    }

    // Side panel mounting holes
    for(p = [[4, -5], [4, 5]]) {
      translate([0, p[1], p[0]]) {
        rotate([0, 90, 0]) {
          cylinder(h = size[0] + 0.1, d = 4, center = true, $fn = 16);
        }
      }
    }
  }
}

DoorSwitchMount();
