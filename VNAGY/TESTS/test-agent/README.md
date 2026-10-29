# VNAGY Canonical Architecture: Formal Verification, Memory Resiliency, and Empirical Validation

Official verification artifacts, formal specifications, and empirical test suites for the **VNAGY Canonical System Architecture** — a high-assurance, offline-first security framework engineered in Rust and backed by mathematical proofs.

---

## Overview

This repository contains the complete verification suite, EasyCrypt and TLA+ formal specifications, and automated execution logs for the VNAGY framework. Engineered to enforce absolute determinism, zero hardcoding, and strict memory safety, the codebase is validated against a rigorous multi-tier formal and empirical verification suite.

---

## Empirical & Formal Verification Suite

### 1. Formal Security Verification (`EasyCrypt & Core Pillars`)
* **Focus:** Pre-implementation mathematical proofs of core system invariants.
* **Result:** Formally verified via EasyCrypt using Hoare triples, establishing provable lemmas for hardware OS entropy (`entropy_pillar_invariant`), 1000-cycle memory zero-leak (`memory_zero_leak_theorem`), and fail-secure boundary enforcement (`fail_secure_boundary_theorem`).

### 2. Multi-Channel Perimeter Cryptographic Audit (`security_perimeter.rs`)
* **Focus:** Zero hardcoding and strict telemetry envelope compliance.
* **Result:** Exhaustive audit across mandatory target packs (`LPack`, `CPack`, and `PPack`) initialized via runtime OS entropy. Enforces rigorous security flags (`is_secured: true`), strict 12-byte nonces, and ciphertext bounds strictly within the 75–90 byte dynamic envelope.

### 3. Fail-Secure Negative Boundary & Resilience Testing (`negative_boundary_test.rs`)
* **Focus:** Attack resistance and malformed key injection defense.
* **Result:** Automated verification confirms absolute resistance against zeroed or bypassed key injections, guaranteeing immediate fail-secure containment rather than silent error propagation.

### 4. Deterministic 1000-Cycle Memory Scope & Zero-Leak Audit (`memory_audit.rs`)
* **Focus:** Memory safety under iterative operational stress.
* **Result:** Validates 1,000 consecutive allocation-drop cycles. Leverages strict Rust scoping and drop semantics to guarantee zero lingering traces and automatic secure cleanup of sensitive materials.

### 5. Production Execution, Cryptographic Telemetry & Resource Footprint (`agent-core`)
* **Focus:** Operational performance and provable resource bounds.
* **Result:** 
  * **Cryptographic Telemetry:** Real-time encryption via ChaCha20-Poly1305 AEAD across target packs.
  * **Resource Efficiency:** Formally bounded via TLA+ (< 50 MB threshold), executing in production at an ultra-lean **~2.5 MB RAM (2536 KB RSS)** with **0.00 seconds** wall-clock execution time and **zero major page faults**.

---

## License & Intellectual Property

All formal specifications, empirical test suites, and software artifacts in this repository are the exclusive intellectual property of the author and are published under the following terms:

* **Author:** Viktorija Nađ, univ. mag. educ. phys. et cs
* **License:** [CC BY-NC-ND 4.0](https://creativecommons.org/licenses/by-nc-nd/4.0/) (Creative Commons Attribution-NonCommercial-NoDerivatives 4.0 International)

*Commercial use and the creation of derivative works or modifications are strictly prohibited without explicit prior written consent from the author.*

---

## Licensing & Intellectual Property Scope Notice

To preserve verification integrity and prevent unauthorized modifications of formal test logic, components of the **VNAGY Framework** operate under distinct licensing models:

* **Research Paper & Documentation (PDF / Zenodo):** Licensed under **[CC BY-NC 4.0](https://creativecommons.org/licenses/by-nc/4.0/)** (Attribution-NonCommercial). Permits academic study, citations, and non-commercial distribution in its original form.
* **Test Suite, EasyCrypt/TLA+ Specifications, & Scripts (GitHub Repository):** Licensed under **[CC BY-NC-ND 4.0](https://creativecommons.org/licenses/by-nc-nd/4.0/)** (Attribution-NonCommercial-NoDerivatives). 

### Enforcement Boundary
While research concepts may be studied, **the formal test suite, execution scripts (`vnagy_*.sh`), and mathematical specifications (`.ec`, `.tla`) must remain intact and unmodified**. Creating derivative test suites, modifying state invariants, or repackaging verification logic is strictly prohibited without explicit written permission from the author (**Viktorija Nađ**).
