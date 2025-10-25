difference() {
  square([70, 70], center = true);

  for(x = [-31, 31]) {
    for(y = [-8.2, 0, 8.2]) {
      translate([x, y]) {
        circle(d = 2, $fn = 16);
      }
    }
  }
}

translate([0, -44]) {
  difference() {
    square([70, 18], center = true);

    for(x = [-25, 0, 25]) {
      translate([x, 0]) {
        circle(d = 6, $fn = 16);
      }
    }
  }
}
