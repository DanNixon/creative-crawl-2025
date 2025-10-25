difference() {
  // Plate
  square([165, 110], center = true);

  // Case mounting holes
  for(x = [-150 / 2, 150 / 2]) {
    for(y = [-75 / 2, 75 / 2]) {
      translate([x, y]) {
        circle(d = 5, $fn = 16);
      }
    }
  }

  // Pi Pico and ESP32 zip tie holes
  translate([-42, 15]) {
    for(x = [-25, 0, 30]) {
      for(y = [-21, 21]) {
        translate([x, y]) {
          square([3, 5], center = true);
        }
      }
    }
  }

  // Breakout board mounting holes
  translate([15, 25]) {
    for(x = [-22 / 2, 22 / 2]) {
      for(y = [-41 / 2, 41 / 2]) {
        translate([x, y]) {
          circle(d = 3.5, $fn = 16);
        }
      }
    }
  }

  // Wago mounting holes
  translate([15, -25]) {
    for(x = [-18 / 2, 18 / 2]) {
      for(y = [-41 / 2, 41 / 2]) {
        translate([x, y]) {
          circle(d = 3.5, $fn = 16);
        }
      }
    }
  }

  // Wire retention zip tie holes
  translate([50, 0]) {
    for(x = [-10, 0, 10]) {
      for(y = [-30, -15, 0, 15, 30]) {
        translate([x, y]) {
          circle(d = 3.5, $fn = 16);
        }
      }
    }
  }
}
