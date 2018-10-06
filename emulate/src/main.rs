extern crate ndarray;
extern crate num_complex;
extern crate core;

use num_complex::Complex;
use core::fmt::{Display, Error, Formatter};

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
    qubits: usize,
    states: usize,
    wf: Vec<Complex<f64>>
}

impl Entangble {
    pub fn new(qubits: usize) -> Self {
        if qubits > 5 {
            eprintln!("Emulating a quantum computer with {} qubits may not finish in feasible amount of time", qubits)
        }
        let states = 2usize.pow(qubits as u32);
        let wf = vec![one(); states];
        Entangble { qubits, states, wf }
    }
}

impl Display for Entangble {
    fn fmt<'a>(&self, f: &mut Formatter<'a>) -> Result<(), Error> {
        for j in 0 .. self.states {
            write!(f, "|{}> ", to_state_repr_binary(j, self.state))?;
            writeln!(f, "")?;  //TODO @mark:
        }
        Ok(())
    }
}

fn to_state_nrs_binary(mut index: usize, state_cnt: usize) -> Vec<usize> {
    let states_per_subsys = 2;
    let mut nrs = Vec::with_capacity(state_cnt);
    for k in 0 .. state_cnt {
        nrs.append(index % states_per_subsys);
        index /= states_per_subsys;
    }
    nrs.reverse();
    nrs
}

fn to_state_repr_binary(index: usize, state_cnt: usize) -> String {
    to_state_nrs_binary(index, state_cnt).iter()
        .map(|nr| format!("{}", nr))
        .collect().join("")
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
