use std::str::FromStr;
use num::Complex;

/// try to determine if 'c' is in mandelbrot set, using at most 'limit'
/// iterations to decide
///
/// if 'c' is not a member, return 'Some(i)', where 'i' is the number of
/// iterations it took for 'c' to leave circle of radius 2 centered on
/// the origin. If 'c' seems to be a member (if we reached the iteration
/// limit without being able to prove that 'c' is not a member),
/// return 'None'
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() < 4.0 { //check z in radius of 2. norm_sqr is
            // faster than checking z root_sq
            return Some(i);
        }
        z = z * z + c;
    }
    None
}

fn parse_pair <T: FromStr>(s: &str, separator: char) -> Option<(T,T)>{
    match s.find(separator) {
        None => None,
        Some(index) => {
            match  (T::from_str(&s[..index]), T::from_str(&s[index+1..]))
            {
                (Ok(l), Ok( r )) => Some((l,r)),
                _=> None
            }
        }
    }
}


fn parse_complex (s: &str) -> Option<Complex<f64>> {
    match parse_pair (s,','){
        Some((l,r)) => Some (Complex {re:l, im:r}),
        None => None
    }

}

/**
D - upper left
|---------|
|         |
|         |
|---------| B - lower right
 **/

/// given row,col of a pixel in the output image, returning the
/// corresponding point on complex plane.
///
/// 'bounds' is a pair giving the width and height of the image in pixels
/// 'pixel' is a (column, row) pair indicating a particular pixel in that image
/// on the complex plain designating the area our image covers.
fn pixel_to_point(  bounds: (usize, usize),
                    pixel: (usize,usize),
                    upper_left: Complex<f64>,
                    lower_right: Complex<f64> )
    -> Complex<f64> {
    let (width, height) = (lower_right.re - upper_left.re, upper_left.im - lower_right.im);
    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        /// subtraction (down) : pixel.1 increases as we go down,
        /// but imaginary component increases as we go up
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64
    }
}

/// Render a rectangle of Mandelbrot set into a buffer of pixels
fn render(  pixels: &mut [u8],
            bounds: (usize, usize),
            upper_left: Complex<f64>,
            lower_right: Complex<f64>)
{
    assert!(pixels.len() == bound.0 * bounds.1);

    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);

            pixels[row * bounds.0 + column] = match escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8
            };
        }
    }

}


fn main() {
    
    escape_time(Complex { re: 0.30, im: 0.1 }, 20);
    let my_complex = Complex::new(1,2);
    let another = my_complex.scale(3);

    println!("original: {} , scaled: {}", my_complex, another );
}

#[test]
fn test_parse_pair(){
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>("10,20",','), Some((10,20)));
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5,1.5)));
    assert_eq!(parse_pair::<f64>("0.5|1.5", '|'), Some((0.5,1.5)));
    assert_eq!(parse_pair::<f64>("0.5|1.5", '\\'), None);
    assert_eq!(parse_complex("0.65,-2.5"), Some(Complex{re: 0.65,im:-2.5}));
    assert_eq!(parse_complex(",-2.5"), None);

}

#[test]
fn test_pixel_to_point(){
    assert_eq!(pixel_to_point((100,200),(25,175),Complex{re: -1.0, im: 1.0},Complex{re: 1.0, im: -1.0}),
               Complex{ re: -0.5, im : -0.75});
}


