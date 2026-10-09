# VNAGY Canonical System Architecture — Verification & Artifact Package

Official verification artifacts, formal specifications, and empirical test suites for the **VNAGY Canonical System Architecture** — a high-assurance, offline-first security framework engineered in Rust.

---

## Overview

This repository contains the complete verification suite, TLA+ formal specifications, and automated reproducibility logs for the VNAGY framework. Engineered to enforce absolute determinism, zero dynamic execution, and global state elimination, the codebase is validated against a rigorous 5-part industrial verification suite.

---

## Empirical Verification Suite

### 1. TLA+ Formal Verification (`Test 1`)
- **Focus:** Mathematical state-space and temporal safety verification.
- **Result:** Formally proved through the TLC Model Checker that scoring outputs (`actionTriggered = FALSE`) remain strictly advisory and isolated prior to code execution, preventing unauthorized automated mutations.

### 2. Architectural Linting & Static Analysis (`Test 2`)
- **Focus:** Enforcement of strict structural determinism and minimalism.
- **Result:** Confirmed **zero** traces of dynamic reflection, dynamic memory allocation, implicit type coercion, or shared mutable global state.

### 3. Reproducible Builds (`Test 3`)
- **Focus:** Cryptographic build artifact verification.
- **Result:** Dual cryptographic SHA-256 checksum validation (`sha256sum`) verified that compiled artifacts are 100% deterministic and reproducible across independent runs.

### 4. Fault Injection & Fuzzing (`Test 4`)
- **Focus:** Malformed input handling and parser fault isolation.
- **Result:** Automated fuzzing confirmed that malformed or corrupted TOML configuration structures are safely rejected without exposing panic vectors or state corruption.

### 5. Resource Footprint Profiling (`Test 5`)
- **Focus:** Operational performance under resource-constrained execution.
- **Result:** Verification pipeline executes well within a strict safe upper limit of **< 50MB RAM** (driven primarily by the Java-based TLC model checker). 
- **Edge Runtime:** In production IoT/embedded deployments, VNAGY utilizes a native Rust micro-layer without garbage collection or VM overhead, operating at a fraction of this memory footprint.

---

## License & Intellectual Property

All formal specifications, empirical test suites, and software artifacts in this repository are the exclusive intellectual property of the author and are published under the following terms:

- **Author:** Viktorija Naď
- **License:** [CC BY-NC-ND 4.0](https://creativecommons.org/licenses/by-nc-nd/4.0/) (Creative Commons Attribution-NonCommercial-NoDerivatives 4.0 International)

*Commercial use and the creation of derivative works or modifications are strictly prohibited without explicit prior written consent from the author.*
