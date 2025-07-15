#include <iostream>

int main() {
    int image_width = 256;
    int image_height = 256;
    
    std::cout << "P3\n" << image_width << ' ' << image_height << "\n255\n";

    for (int i = 0; i < image_height; i++) {
        std::clog << "\rScanlines remaining: " << (image_height - i) << std::flush;
        for (int j = 0; j < image_width; j++) {
            double r = double(j) / (image_width-1);
            double g = double(i) / (image_height-1);
            double b = 0.0;

            int ir = int(255.999 * r);
            int ig = int(255.999 * g);
            int ib = int(255.999 * b);

            std::cout << ir << ' ' << ig << ' ' << ib << '\n';
        }
    }

    std::clog << "\rDone.        \n";
}
