# ternary-phase

Phase relationships between ternary oscillators. Phase wrapping, phase difference, Kuramoto coherence order parameter $R$, phase-lock detection, and anti-phase identification — for populations of {-1, 0, +1} agents mapped onto a 3-sector phase cycle.

## Why It Matters

Synchronization is one of the most universal phenomena in nature — from firefly flashing to power grid stability to neural oscillations. Ternary phase dynamics extend this to systems with three natural states, where phase space divides into three sectors:

- **Sector 0 (0 to 2π/3):** maps to +1 (constructive/active)
- **Sector 1 (2π/3 to 4π/3):** maps to 0 (neutral/transitional)
- **Sector 2 (4π/3 to 2π):** maps to -1 (inhibitory/quiescent)

This three-state cycle naturally models circadian rhythms, ternary logic clocks, multi-phase power systems, and oscillating agent populations where agents cycle through active/neutral/inhibitory phases.

## How It Works

### Phase Representation

Phase $\phi \in [0, 2\pi)$, wrapped modulo $2\pi$:

$$\text{wrap}(\phi) = \phi \bmod 2\pi, \quad \text{adjusted to } [0, 2\pi)$$

### Phase Difference

Signed difference wrapped to $[-\pi, \pi)$:

$$\Delta\phi = ((\phi_A - \phi_B) + \pi) \bmod 2\pi - \pi$$

This ensures the difference always takes the shortest angular path.

### Phase Lock

Two oscillators are phase-locked if:

$$|\Delta\phi| < \epsilon$$

for threshold $\epsilon$ (typically 0.1-0.5 radians).

**Anti-phase** ($\pi$ apart):

$$||\Delta\phi| - \pi| < \epsilon$$

### Kuramoto Order Parameter

The phase coherence of a population of $N$ oscillators:

$$R = \left|\frac{1}{N}\sum_{j=1}^{N} e^{i\theta_j}\right| = \sqrt{\left(\frac{1}{N}\sum_j \cos\theta_j\right)^2 + \left(\frac{1}{N}\sum_j \sin\theta_j\right)^2}$$

- $R = 1$: perfect synchrony (all phases aligned)
- $R = 0$: incoherent (phases uniformly distributed)
- $R \in (0, 1)$: partial synchrony

**Complexity:** O($N$) — single pass over phases.

### In-Phase Ratio

Fraction of a population locked to a reference phase:

$$f_{\text{lock}} = \frac{|\{j : |\Delta(\theta_j, \phi_{\text{ref}})| < \epsilon\}|}{N}$$

### Phase Velocity

Rate of phase change:

$$\omega = \frac{\Delta\phi}{\Delta t}$$

At $\Delta t = 0$, returns 0 (avoiding division by zero).

### Ternary Sector Mapping

$$\text{trit}(\phi) = \begin{cases} +1 & \text{if } 0 \leq \phi < 2\pi/3 \\ 0 & \text{if } 2\pi/3 \leq \phi < 4\pi/3 \\ -1 & \text{if } 4\pi/3 \leq \phi < 2\pi \end{cases}$$

## Quick Start

```rust
use ternary_phase::*;
use std::f64::consts::TAU;

// Phase wrapping
assert!((phase_wrap(TAU + 0.5) - 0.5).abs() < 1e-10);
assert!((phase_wrap(-0.5) - (TAU - 0.5)).abs() < 1e-10);

// Phase difference (shortest path)
let d = phase_difference(0.1, TAU - 0.1);
assert!((d - 0.2).abs() < 1e-6); // wraps around

// Phase lock detection
assert!(phase_lock(1.0, 1.01, 0.1));   // locked
assert!(!phase_lock(0.0, std::f64::consts::PI, 0.1)); // not locked

// Kuramoto coherence
let synced = vec![0.0, 0.01, 0.02, 0.03];
assert!(phase_coherence(&synced) > 0.99); // R ≈ 1

let uniform: Vec<f64> = (0..12).map(|i| i as f64 * TAU / 12.0).collect();
assert!(phase_coherence(&uniform) < 0.3); // R ≈ 0

// Anti-phase detection
assert!(anti_phase(0.0, std::f64::consts::PI, 0.1));

// In-phase ratio
let phases = vec![1.0, 1.01, 1.02, 3.0];
assert!((in_phase_ratio(&phases, 1.0, 0.1) - 0.75).abs() < 0.01);

// Ternary sector mapping
let p = Phase::new(0.0);
assert_eq!(p.to_trit(), Trit::Pos);
```

## API

| Function | Description |
|---|---|
| `Phase::new(f64)` | Wrapped phase value |
| `.to_trit()` | Map to dominant ternary sector |
| `phase_wrap(p) → f64` | Wrap to [0, 2π) |
| `phase_difference(a, b) → f64` | Signed Δ in [-π, π) |
| `phase_lock(a, b, threshold) → bool` | |Δ| < threshold |
| `anti_phase(a, b, threshold) → bool` | Δ ≈ ±π |
| `phase_velocity(Δφ, Δt) → f64` | Angular velocity |
| `in_phase_ratio(phases, ref, threshold) → f64` | Fraction locked to reference |
| `phase_coherence(phases) → f64` | Kuramoto $R \in [0,1]$ |

## Architecture Notes

Phase dynamics implement the **γ + η = C** identity through the angular distribution of oscillator populations. When $R \approx 1$ (synchronized), the entire population occupies one ternary sector — either all constructive (γ-dominated), all inhibitory (η-dominated), or all neutral. The conserved quantity $C = N$ is distributed among the three phase sectors.

As $R$ decreases, the population spreads across all three sectors, and the system passes through a phase transition described by the Kuramoto model:

$$\frac{d\theta_i}{dt} = \omega_i + \frac{K}{N}\sum_j \sin(\theta_j - \theta_i)$$

The critical coupling $K_c$ for ternary phase transitions is modified relative to continuous Kuramoto because the three-sector discretization introduces additional locking thresholds at $\pm 2\pi/3$.

## References

- Kuramoto, Y. (1984). *Chemical Oscillations, Waves, and Turbulence.* Springer.
- Strogatz, S. H. (2000). *From Kuramoto to Crawford.* Physica D, 143(1-4).
- Pikovsky, A. et al. (2003). *Synchronization.* Cambridge University Press.
- Acebrón, J. A. et al. (2005). *The Kuramoto Model: A Simple Paradigm for Synchronization Phenomena.* Rev. Mod. Phys., 77(1).

## License

MIT
