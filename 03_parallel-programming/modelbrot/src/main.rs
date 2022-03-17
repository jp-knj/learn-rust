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