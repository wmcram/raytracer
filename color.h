#ifndef COLOR_H
#define COLOR_H

#include "vec3.h"

#include <iostream>

using color = vec3;

void write_color(std::ostream &out, const color &pixel_color) {
  double r = pixel_color.x();
  double g = pixel_color.y();
  double b = pixel_color.z();

  int rbyte = int(255.99 * r);
  int gbyte = int(255.99 * g);
  int bbyte = int(255.99 * b);

  out << rbyte << ' ' << gbyte << ' ' << bbyte << '\n';
}

#endif
