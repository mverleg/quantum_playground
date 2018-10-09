
//TODO @mark: allow dead code during dev
#![allow(dead_code)]

extern crate ndarray;
extern crate num_complex;
extern crate core;

use num_complex::Complex;
use core::fmt::{Display, Error, Formatter};

pub fn zero() -> Complex<f64> {
    // Cannot import Complex::one() for some reason
    return Complex::new(0., 0.)
}

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
        assert!(qubits > 0);
        if qubits > 5 {
            eprintln!("Emulating a quantum computer with {} qubits may not finish in feasible amount of time", qubits)
        }
        let states = 2usize.pow(qubits as u32);
        let mut wf = vec![zero(); states];
        wf[0] = one();
        Entangble { qubits, states, wf }
    }
}

impl Display for Entangble {
    fn fmt<'a>(&self, f: &mut Formatter<'a>) -> Result<(), Error> {
        println!("{}-state entangled quantum system:", self.qubits);
        for j in 0 .. self.states {
            writeln!(f, " |{}> {}  {:.3} + {:.3}i",
                    to_state_repr_binary(j, self.qubits),
                     norm_ascii_log_magnitude(self.wf[j].norm(), 8),
                     self.wf[j].re, self.wf[j].im
            )?;
        }
        Ok(())
    }
}

/// Convert a state index to a vector of state numbers, e.g. `[0, 0, 1, 0]` which'd be index `2`.
fn to_state_nrs_binary(mut index: usize, subsys_cnt: usize) -> Vec<usize> {
    let states_per_subsys = 2;
    let mut nrs = Vec::with_capacity(subsys_cnt);
    for _ in 0 .. subsys_cnt {
        nrs.push(index % states_per_subsys);
        index /= states_per_subsys;
    }
    nrs.reverse();
    nrs
}

/// Print a mixed state, e.g. `"0010"` for `|0> x |0> x |1> x |0>` which'd be index `2`.
fn to_state_repr_binary(index: usize, subsys_cnt: usize) -> String {
    to_state_nrs_binary(index, subsys_cnt).iter()
        .map(|nr| format!("{}", nr))
        .collect::<Vec<_>>().join("")
}

/// Show the magnitude as a (0, 1) number as ****-symbols
fn norm_ascii_log_magnitude(magnitude: f64, steps: u8) -> String {
    if magnitude < 0. {
        return "NEGATIVE".to_owned()
    }
    if magnitude > 1. {
        return "TOO BIG ".to_owned()
    }
    (1 .. steps + 1)
        .map(|k| if magnitude > (0.5f64).powf(k as f64) { "*" } else { " " })
        .collect::<Vec<_>>().join("")
}

/// System consisting of multiple entangled ensembles
// TODO LATER
struct System {
}

pub fn main() {
    let qsys = Entangble::new(4);
    println!("{}", qsys);
}
