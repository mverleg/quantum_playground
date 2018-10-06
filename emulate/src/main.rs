extern crate ndarray;
extern crate num_complex;

use num_complex::Complex;

pub fn one() -> Complex<f64> {
    // Cannot import Complex::one() for some reason
    return Complex::new(1., 0.)
}

/// Qubit (qbit)
/// There are really only 3 degrees of freedom, since this should be normalized
//TODO @mark: remove this? I don't think it's useful to consider isolated qubits in an entangled system
struct Qubit(Complex<f64>, Complex<f64>);

/// Entangled (sub)system
/// Combination of 'entangled' and 'ensemble', haha!
// Storage order: |0..00>, |0..01>, |0..10>, ..., |1..11>
struct Entangble {
    n: usize,
    wf: Vec<Complex<f64>>
}

impl Entangble {
    pub fn new(size: usize) -> Self {
        if size > 5 {
            eprintln!("Emulating a quantum computer with {} qubits may not finish in feasible amount of time", size)
        }
        let wf = vec![one(); 2usize.pow(size as u32)];
        Entangble { n: size, wf }
    }
}

/// System consisting of multiple entangled ensembles
// TODO LATER
struct System {

}

pub fn main() {
    let qsys = Entangble::new(4);

//    let xs = Array::from_vec(vec![Complex::new(1., 1.), Complex::new(3., -1.)]);
//    println!("xs: {}", xs);  //TODO: mark (temporary)
//    assert!(xs.dot(&xs) == Complex::new(12., 0.));
}

//#[cfg(test)]
//mod tests {
//    #[test]
//    fn it_works() {
//        assert_eq!(2 + 2, 4);
//    }
//}
