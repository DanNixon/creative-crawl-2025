module Washer(inner, outer) {
  difference() {
    circle(d = outer, $fn = 16);
    circle(d = inner, $fn = 16);
  }
}

for(i = [0:4]) {
  translate([i * 7.2, 0]) {
    Washer(3.2, 7);
  }
}

for(i = [0:4]) {
  translate([i * 12.2, 10]) {
    Washer(5.2, 12);
  }
}
