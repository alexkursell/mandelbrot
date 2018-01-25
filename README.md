# mandelbrot
Command line tool to generate grayscale mandelbrot visualizations. Written in Rust.

## Usage
mandelbrot requires 6 arguments:
```
    <X-TOPLEFT> - The real component of the point at the top left of the image. This is a decimal.
    <Y-TOPLEFT> - The imaginary component of the point at the top left of the image. This is a decimal.
    <SCALE>     - The horizontal width of the view of the mandelbrot set. This is a decimal.
    <XRES>      - The horizontal resolution of the image. This is a positive integer.
    <YRES>      - The vertical resolution of the image. This is a positive integer.
    <FILE>      - The name of the output file. Should be a .png.
```
    
Do not run with ```cargo run```, as negative numbers will be interpreted as flags and the program will not run. 
Instead, run ```cargo build --release```, cd to ```/target/release/``` and use the ```mandelbrot``` executable directly.
Note the ```--``` in the examples below. It prevents negative numbers from being interpreted as flags.

## Examples

```./mandelbrot -- -2.0 2.0 4.0 1024 1024 set.png```

Produces an image of the whole set:

![Image](https://raw.githubusercontent.com/alexkursell/mandelbrot/master/examples/set.png)

---

```./mandelbrot -- -0.66 0.4508 0.015 1920 1080 img.png```

Produces my desktop wallpaper :-)

![Image](https://raw.githubusercontent.com/alexkursell/mandelbrot/master/examples/img.png)
  
