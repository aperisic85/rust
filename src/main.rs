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

fn main() {
    println!("Hello, world!");
    escape_time(Complex { re: 0.30, im: 0.1 }, 20);
}
//tst
#[test]
fn test_parse_pair(){
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>("10,20",','), Some((10,20)));
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5,1.5)));
    assert_eq!(parse_pair::<f64>("0.5|1.5", '|'), Some((0.5,1.5)));
    assert_eq!(parse_pair::<f64>("0.5|1.5", '\\'), None);


}

