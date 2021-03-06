//! Noise Propagation Estimator Module
//! * Contains material needed to estimate the growth of the noise when performing homomophic computation

pub mod cross;
pub mod lwe;
pub mod rlwe;

pub use cross::Cross;
pub use lwe::LWE;
pub use rlwe::RLWE;

/// Computes the variance of the error distribution after the addition of two uncorrelated ciphertexts
/// Arguments
/// * `var_ct1` - noise variance of the first ciphertext
/// * `var_ct2` - noise variance of the second ciphertext
/// Output
/// * the variance of the sum of the first and the second ciphertext
pub fn add_ciphertexts(var_ct1: f64, var_ct2: f64) -> f64 {
    return var_ct1 + var_ct2;
}

/// Computes the number of bits affected by the noise with a variance var describing a normal distribution
/// takes into account the number of bits of the integers
pub fn nb_bit_from_variance_99(var: f64, torus_bit: usize) -> usize {
    // compute sigma
    let sigma: f64 = f64::sqrt(var);

    // the constant to get 99% of the normal distribution
    let z: f64 = 3.;
    let tmp = torus_bit as f64 + f64::log2(sigma * z);
    if tmp < 0. {
        // means no bits are affected by the noise in the integer representation (discrete space)
        return 0usize;
    } else {
        return tmp.ceil() as usize;
    }
}
