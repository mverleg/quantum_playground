
//TODO @mark: allow dead code during dev
#![allow(dead_code)]

extern crate core;
extern crate ndarray;
extern crate num_complex;
extern crate rand;

use core::fmt::{Display, Error, Formatter};
use num_complex::Complex;
use rand::Rng;

//TODO @mark: turn State into a type (which wraps a usize and is printable as |010>)

pub fn zero() -> Complex<f64> {
    // Cannot import Complex::one() for some reason
    return Complex::new(0., 0.)
}

pub fn one() -> Complex<f64> {
    // Cannot import Complex::one() for some reason
    return Complex::new(1., 0.)
}

pub fn weighted_choice<R: Rng>(weights: &Vec<f64>, mut rng: R) -> usize {
    assert!((weights.iter().sum::<f64>() - 1.) < 1e-8);
    let mut cumsum = 0.;
    let choice = rng.gen();
    for (k, w) in weights.iter().enumerate() {
        cumsum += w;
        if cumsum >= choice {
            return k
        }
    }
    unreachable!();
}

pub trait QuantumState {
    /// Observe (and collapse) the whole system. The system itself will be in a pure quantum state,
    /// and the integer values per state will be returned.
    fn observe(&mut self) -> usize;

//    /// Observe the system and return the classical probability distribution.
//    /// This cannot be done on a real quantum computer, but is one of the perks of an emulator.
//    fn spy_probabilities(&mut self) -> Vec<u8>;
}

/// Qubit (qbit)
/// There are really only 3 degrees of freedom, since this should be normalized
//TODO @mark: remove this? I don't think it's useful to consider isolated qubits in an entangled system
struct Qubit(Complex<f64>, Complex<f64>);

/// Entangled (sub)system
/// Combination of 'entangled' and 'ensemble', haha!
// Storage order: |0..00>, |0..01>, |0..10>, ..., |1..11>
struct Entangble<R: Rng> {
    qubits: usize,
    states: usize,
    wf: Vec<Complex<f64>>,
    rng: R,
}

impl <R: Rng> Entangble<R> {
    pub fn new(qubits: usize, rng: R) -> Self {
        assert!(qubits > 0);
        if qubits > 5 {
            eprintln!("Emulating a quantum computer with {} qubits may not finish in feasible amount of time", qubits)
        }
        let states = 2usize.pow(qubits as u32);
        let wf = vec![zero(); states];
        let mut ent = Entangble { qubits, states, wf, rng };
        ent.set_pure(0);
        ent
    }

    /// Collapse into one pure state
    fn set_pure(&mut self, index: usize) {
        for wfi in self.wf.iter_mut() {
            *wfi = zero();
        }
        self.wf[index] = one();
    }

    /// Calculate the classical probabilities (which one wouldn't be able to do on a real quantum computer, but can be done on the emulator).
    fn calc_probs(&self) -> Vec<f64> {
        self.wf.iter().map(|v| v.norm()).collect()
    }

    /// Check that the total occupation is still unity
    pub fn check_norm(&self) {
        assert!((self.calc_probs().iter().sum::<f64>() - 1.) < 1e-8);
    }
}

impl<R: Rng> QuantumState for Entangble<R> {
    fn observe(&mut self) -> usize {
        let probs = self.calc_probs();
        let pick = weighted_choice(&probs, &mut self.rng);
        self.set_pure(pick);
        pick
        //TODO @mark: partial observation
    }
}

impl<R: Rng> Display for Entangble<R> {
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
    let mut rng = rand::thread_rng();
    let mut qsys = Entangble::new(4, rng);
    println!("{}", qsys);
    println!("{:?}", qsys.observe());
    println!("{}", qsys);
}
