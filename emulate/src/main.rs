extern crate ndarray;
extern crate num_complex;

use ndarray::Array;
use num_complex::Complex;

pub fn main() {
    let xs = Array::from_vec(vec![Complex::new(1., 1.), Complex::new(3., -1.)]);
    println!("xs: {}", xs);  //TODO: mark (temporary)
    assert!(xs.dot(&xs) == Complex::new(12., 0.));
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
