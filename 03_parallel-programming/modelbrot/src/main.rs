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