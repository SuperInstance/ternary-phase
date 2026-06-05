//! # ternary-phase
//! Phase relationships between ternary oscillators.

#![forbid(unsafe_code)]

use std::f64::consts::{PI, TAU};

/// A ternary value: Negative (-1), Zero (0), or Positive (+1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Trit {
    Neg,
    Zero,
    Pos,
}

impl Trit {
    pub fn to_i8(self) -> i8 {
        match self {
            Trit::Neg => -1,
            Trit::Zero => 0,
            Trit::Pos => 1,
        }
    }

    pub fn from_i8(v: i8) -> Self {
        match v {
            -1 => Trit::Neg,
            0 => Trit::Zero,
            _ => Trit::Pos,
        }
    }
}

/// Phase value mapped to a ternary cycle (0 to 2π).
#[derive(Debug, Clone, Copy)]
pub struct Phase(f64);

impl Phase {
    pub fn new(raw: f64) -> Self {
        Self(phase_wrap(raw))
    }

    pub fn value(&self) -> f64 {
        self.0
    }

    /// Map phase to the dominant trit in a 3-sector cycle.
    pub fn to_trit(&self) -> Trit {
        let sector = ((self.0 / TAU) * 3.0).floor() as i32 % 3;
        match sector {
            0 => Trit::Pos,
            1 => Trit::Zero,
            _ => Trit::Neg,
        }
    }
}

/// Wrap phase into [0, 2π).
pub fn phase_wrap(p: f64) -> f64 {
    let r = p % TAU;
    if r < 0.0 { r + TAU } else { r }
}

/// Compute signed phase difference a - b wrapped to [-π, π).
pub fn phase_difference(a: f64, b: f64) -> f64 {
    let mut d = (a - b) % TAU;
    if d > PI { d -= TAU; }
    if d < -PI { d += TAU; }
    d
}

/// Detect phase lock: true if |difference| < threshold.
pub fn phase_lock(a: f64, b: f64, threshold: f64) -> bool {
    phase_difference(a, b).abs() < threshold
}

/// Rate of phase change (velocity) = Δphase / Δtime.
pub fn phase_velocity(delta_phase: f64, delta_time: f64) -> f64 {
    if delta_time == 0.0 { 0.0 } else { delta_phase / delta_time }
}

/// Fraction of a population in-phase with a reference.
pub fn in_phase_ratio(phases: &[f64], reference: f64, threshold: f64) -> f64 {
    if phases.is_empty() { return 0.0; }
    let locked = phases.iter().filter(|&&p| phase_lock(p, reference, threshold)).count();
    locked as f64 / phases.len() as f64
}

/// Check if two oscillators are anti-phase (π apart within threshold).
pub fn anti_phase(a: f64, b: f64, threshold: f64) -> bool {
    (phase_difference(a, b).abs() - PI).abs() < threshold
}

/// Kuramoto-style phase coherence order parameter R ∈ [0, 1].
/// R = |1/N * Σ e^(iθ_j)|
pub fn phase_coherence(phases: &[f64]) -> f64 {
    if phases.is_empty() { return 0.0; }
    let n = phases.len() as f64;
    let sum_cos: f64 = phases.iter().map(|&p| p.cos()).sum();
    let sum_sin: f64 = phases.iter().map(|&p| p.sin()).sum();
    let r_cos = sum_cos / n;
    let r_sin = sum_sin / n;
    (r_cos * r_cos + r_sin * r_sin).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::TAU;

    #[test]
    fn test_phase_wrap_identity() {
        assert!((phase_wrap(1.0) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_phase_wrap_above_tau() {
        assert!((phase_wrap(TAU + 0.5) - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_phase_wrap_negative() {
        assert!((phase_wrap(-0.5) - (TAU - 0.5)).abs() < 1e-10);
    }

    #[test]
    fn test_phase_difference_zero() {
        let d = phase_difference(1.0, 1.0);
        assert!(d.abs() < 1e-10);
    }

    #[test]
    fn test_phase_difference_half_pi() {
        let d = phase_difference(1.5, 1.0);
        assert!((d - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_phase_difference_wraps() {
        let d = phase_difference(0.1, TAU - 0.1);
        assert!(d > 0.0); // should wrap to ~0.2
        assert!((d - 0.2).abs() < 1e-6);
    }

    #[test]
    fn test_phase_lock_true() {
        assert!(phase_lock(1.0, 1.01, 0.1));
    }

    #[test]
    fn test_phase_lock_false() {
        assert!(!phase_lock(0.0, PI, 0.1));
    }

    #[test]
    fn test_phase_velocity_basic() {
        let v = phase_velocity(TAU, 1.0);
        assert!((v - TAU).abs() < 1e-10);
    }

    #[test]
    fn test_phase_velocity_zero_time() {
        assert_eq!(phase_velocity(1.0, 0.0), 0.0);
    }

    #[test]
    fn test_in_phase_ratio_all() {
        let phases = vec![1.0, 1.01, 0.99, 1.02];
        let r = in_phase_ratio(&phases, 1.0, 0.1);
        assert!((r - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_in_phase_ratio_half() {
        let phases = vec![1.0, 1.01, PI, PI + 0.01];
        let r = in_phase_ratio(&phases, 1.0, 0.1);
        assert!((r - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_in_phase_ratio_empty() {
        assert_eq!(in_phase_ratio(&[], 0.0, 0.1), 0.0);
    }

    #[test]
    fn test_anti_phase_true() {
        assert!(anti_phase(0.0, PI, 0.1));
    }

    #[test]
    fn test_anti_phase_false() {
        assert!(!anti_phase(0.0, 0.5, 0.1));
    }

    #[test]
    fn test_phase_coherence_synced() {
        let phases = vec![0.0, 0.01, 0.02, 0.03];
        let c = phase_coherence(&phases);
        assert!(c > 0.99);
    }

    #[test]
    fn test_phase_coherence_uniform() {
        // Uniformly spaced -> low coherence
        let phases: Vec<f64> = (0..12).map(|i| i as f64 * TAU / 12.0).collect();
        let c = phase_coherence(&phases);
        assert!(c < 0.3);
    }

    #[test]
    fn test_phase_coherence_empty() {
        assert_eq!(phase_coherence(&[]), 0.0);
    }

    #[test]
    fn test_phase_new_wraps() {
        let p = Phase::new(TAU + 0.5);
        assert!((p.value() - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_phase_to_trit_pos() {
        let p = Phase::new(0.0);
        assert_eq!(p.to_trit(), Trit::Pos);
    }
}
