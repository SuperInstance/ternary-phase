# ternary-phase

**Phase relationships between ternary oscillators. In tune or out of phase?**

When two oscillators are "in sync," their phases align — both at the same point in their cycle at the same time. But "in sync" isn't binary. Two oscillators can be perfectly aligned, perfectly opposed (180° out of phase), or anywhere in between. Phase relationships are the *geometry* of synchronization.

This crate maps continuous phase angles to ternary sectors: each 120° slice of the cycle maps to one ternary value. Phase coherence measures how aligned a group of oscillators is. Phase locking detects when oscillators maintain a fixed phase relationship. The result is a *ternary phase space* where synchronization is a geometric property, not just a binary state.

## What's Inside

- **`Phase`** — wrapped phase value in [0, 2π). `to_trit()` maps to ternary sector
- **`Phase::coherence(phases)`** — Kuramoto-style order parameter for ternary phases. 1 = aligned, 0 = random
- **`Phase::locking_error(phase_a, phase_b)`** — how far are two oscillators from being phase-locked?
- **`Phase::is_locked(a, b, tolerance)`** — are they locked within tolerance?
- **`Phase::difference(a, b)`** — the shortest angular distance between two phases
- **`Phase::advance(phase, omega, dt)`** — step a phase forward by angular velocity × time

## Quick Example

```rust
use ternary_phase::*;

// Two oscillators: one at 0°, one at 120°
let a = Phase::new(0.0);
let b = Phase::new(2.0 * PI / 3.0);

// Their ternary values
assert_eq!(a.to_trit(), Trit::Pos);   // 0° → +1
assert_eq!(b.to_trit(), Trit::Zero);  // 120° → 0

// Phase difference
let diff = Phase::difference(a, b);
// 120° — one sector apart

// Are they phase-locked?
assert!(Phase::is_locked(a, b, PI / 3.0)); // within 60° tolerance

// Coherence of a group
let phases = vec![Phase::new(0.0), Phase::new(0.1), Phase::new(0.2)];
let coh = Phase::coherence(&phases);
assert!(coh > 0.9); // nearly aligned → high coherence
```

## The Deeper Truth

**120° sectors create exactly three phase relationships.** Two ternary oscillators can be in-phase (same sector), adjacent (one sector apart), or opposed (two sectors apart). That's it. This is simpler than continuous phase, where the relationship is a continuous angle. In ternary, phase is a *categorical* variable: same, adjacent, or opposed. This makes phase analysis combinatorial rather than geometric — and combinatorial problems are decidable in ways that continuous problems aren't.

The ternary phase circle is Z₃ — the cyclic group of order 3. Phase advancement by one sector is exactly the Z₃ rotation. This means ternary phase dynamics are governed by the same algebra as rock-paper-scissors, ternary addition, and all the other Z₃ structures in the ternary ecosystem.

**Use cases:**
- **Audio synthesis** — phase alignment between oscillators for rich timbres
- **Multi-agent coordination** — are agents in phase or fighting each other?
- **Neural oscillators** — brain rhythm synchronization in ternary models
- **Power systems** — phase relationships in three-phase power (literally Z₃!)
- **Dance/music** — phase relationships between performers

## See Also

- **ternary-kuramoto** — the dynamical system that drives phase synchronization (or fails to)
- **ternary-harmonic** — harmonic relationships between phase-locked oscillators
- **ternary-sync** — Z₃ synchronization primitives
- **ternary-polyrhythm** — phase relationships across different period lengths

## Install

```bash
cargo add ternary-phase
```

## License

MIT
