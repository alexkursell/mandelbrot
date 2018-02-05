extern crate rayon;
extern crate image;
extern crate clap;

use std::ops::{Add, Mul};
use rayon::prelude::*;
use std::fs::File;
use std::path::Path;
use clap::App;

#[derive(Debug, Copy, Clone, Default)]
struct Complex{
    re : f32,
    im : f32 
}


impl Add for Complex{
    type Output = Complex;
    fn add(self, other : Complex) -> Self::Output{
        Complex{re : self.re + other.re, 
                im : self.im + other.im}
    }
}

impl Mul for Complex{
    type Output = Complex;
    fn mul(self, other : Complex) -> Complex{
        Complex{re : self.re * other.re - self.im * other.im,
                im : self.re * other.im + self.im * other.re}
    }
}

#[derive(Default)]
struct MandelView{
    topleft : Complex, //The top left point in the view
    xdelta : f32,      //The x-distance between pixels
    ydelta : f32,      //The y-distance between pixels
    cur : Complex,     //The current point in the iteration
    xres : u32,        //Overall x-resolution of the view
    yres : u32,        //Overall y-resolution of the view
    curx : u32,        //Current pixel x-value in the iteration
    cury : u32,        //Currnet pixel y-value in the iteration
}

impl MandelView{
    fn new(topleft : Complex, botright : Complex, xres : u32, yres : u32) -> MandelView{
        let xdelta : f32 = (botright.re - topleft.re) as f32 / xres as f32;
        let ydelta : f32 = (topleft.im - botright.im) as f32 / yres as f32;

        MandelView{
            topleft : topleft + Complex{re : 0.0, im : xdelta * -1.0},
            cur     : topleft,
            curx    : 0,
            cury    : 0,
            xdelta,
            ydelta,
            xres,
            yres
        }

    }
}

impl Iterator for MandelView{
    type Item = Complex;
    fn next(&mut self) -> Option<Complex>{
        if self.curx == self.xres{         //Reached end of line
            if self.cury == self.yres{     //Reached end of view
                return None
            }

            self.cur.re = self.topleft.re; //Go back to leftmost
            self.cur.im -= self.ydelta;    //Go down one line
            self.cury += 1;                //Record having gone down a line
            self.curx = 1;                 //Record having gone to the start of a line
        }
        else{
            self.cur.re += self.xdelta;    //Move one pixel left
            self.curx += 1;                //Record having done so.
        }

        Some(self.cur)
    }
}

fn mandel_iter(c : Complex) -> u8{
    let mut z = Complex{re: 0.0, im : 0.0};  
    for n in 0..255{
        z = z * z + c;
        //If |z| > 2, this is guaranteed to diverge
        //The faster the value "escapes", the lighter the colour
        if z.im * z.im + z.re * z.re >= 4.0{ 
            return 255 - n;                  
        }
    }

    //0 means it never escaped. Coloured black
    0                                        
}

fn compute_image(m : MandelView, color : bool) -> Vec<u8>{
    let v : Vec<Complex> = m.collect();
    let raw = v.par_iter()
     .map(|c| mandel_iter(*c))          
     .collect();             

    if !color { return raw }

    let mut col : Vec<u8> = Vec::with_capacity(raw.len() * 3);

    //Quick and dirty conversion from grayscale to black+rainbow.
    for v in raw{
        let r = if v < 16 {v * 16} else {255 - (v - 16)};
        let g = if v >= 128 {255 - (v - 128) * 2} else {v * 2};
        let b = v;

        col.push(r);
        col.push(g);
        col.push(b);
    }

    col

}


fn main() {
    let matches = App::new("Mandelbrot Visualizer")
                          .version(env!("CARGO_PKG_VERSION"))
                          .author(env!("CARGO_PKG_AUTHORS"))
                          .about(env!("CARGO_PKG_DESCRIPTION"))
                          .args_from_usage(
                             "<X-TOPLEFT>          'Sets the real component of the top left of the image'
                              <Y-TOPLEFT>          'Sets the imaginary component of the top left of the image'
                              <SCALE>              'Total width of the view'
                              <XRES>               'Horizontal resolution of the output image'
                              <YRES>               'Vertical resolution of the output image'
                              <FILE>               'Name of the output file. Suffix should be .png'

                              -c, --color           'Use 256 colors instead of grayscale.")
                          .get_matches();


    //Try to open the specified filename and set up the encoder.
    let path = Path::new(matches.value_of("FILE").unwrap());
    let file = File::create(&path).unwrap();
    let i = image::png::PNGEncoder::new(file);

    let color = matches.is_present("color");
    let image_type = match color{
        false => image::ColorType::Gray(8),
        true => image::ColorType::RGB(8)
    };

    //Set up resolution.
    let xres = matches.value_of("XRES").unwrap().parse::<u32>().unwrap();
    let yres = matches.value_of("YRES").unwrap().parse::<u32>().unwrap();
    let xry = xres as f32 / yres as f32; //Window size ratio.
    
    //Set up scale variables.
    let xwidth = matches.value_of("SCALE").unwrap().parse::<f32>().unwrap();
    let ywidth = xwidth / xry;

    //Represents the point at the very top-left of the image.
    let tleft  = Complex{
        re : matches.value_of("X-TOPLEFT").unwrap().parse::<f32>().unwrap(),
        im : matches.value_of("Y-TOPLEFT").unwrap().parse::<f32>().unwrap()
    };

    //Represents the point at the very bottom-right of the image.
    let mut bright = Complex{
        re : tleft.re + xwidth, 
        im : tleft.im - ywidth
    };
    bright.im = tleft.im - (bright.re - tleft.re) / xry as f32;

    //Create a new iterator over the points in the view specified.
    let m = MandelView::new(tleft, bright, xres, yres);

    //Compute the actual image and write to file.
    let _ = i.encode(compute_image(m, color).as_slice(), xres, yres, image_type);

}