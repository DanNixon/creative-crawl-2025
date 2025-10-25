use <fonts/Asimovian-Regular.ttf>;

color("purple") {
  translate([0, 0, -1]) {
    //square([320, 480], center = true);
  }
}

r = [0, 0, -28];

linear_extrude(0.5)
color("lightgrey") {
  translate([0, 60]) {
    rotate(r) {
      text("The system", size = 40, halign = "center", valign = "center", font = "Asimovian:style=Regular");
    }
  }

  translate([0, 0]) {
    rotate(r) {
      text("has a", size = 30, halign = "center", valign = "center", font = "Asimovian:style=Regular");
    }
  }

  translate([0, -60]) {
    rotate(r) {
      text("story for you", size = 40, halign = "center", valign = "center", font = "Asimovian:style=Regular");
    }
  }
}
