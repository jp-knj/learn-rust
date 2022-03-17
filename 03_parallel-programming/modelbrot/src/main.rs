use num::Complex;

struct Complex<T> {
    re : T,
    im: T
}

fn complex_square_add_loop(c: Complex<f64>){
    let mut z = Complex{re: 0.0, im: 0.0};
    loop {
        z = z * z + c;
    }
}

use num::Complex;

fn escape_time(c:Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex{ re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_spr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }
    None
}

use std::str::FromStr;

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None
            }
        }
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("",        ','), None);
    assert_eq!(parse_pair::<i32>("10,",     ','), None);
    assert_eq!(parse_pair::<i32>(",10",     ','), None);
    assert_eq!(parse_pair::<i32>("10,20",   ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x",    'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}