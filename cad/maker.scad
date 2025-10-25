module Text2D() {
  difference() {
    text("MAKER", size=50, font="Liberation Sans:style=Bold", halign="center", valign="center");

    h = 20;
    for(p = [
      [-115, h],
      [-43, h],
      [-9, h],
      [40, h],
      [85, h],
    ]) {
      translate(p) {
        #circle(d = 2, $fn = 16);
      }
    }
  }
}

//linear_extrude(3)
Text2D();
