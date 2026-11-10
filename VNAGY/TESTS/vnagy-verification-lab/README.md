# VNAGY Verification Lab (`vnagy-verification-lab`)

Central repository for out-of-repo formal verification, symbolic execution, and mathematical security validation of the VNAGY agent ecosystem.

## Purpose

This directory contains the autonomous verification workspace, executable specifications, and theorem-proving tooling dedicated to systematically evaluating system properties and security invariants against an infinite state space using Satisfiability Modulo Theories (SMT).

## Verification Scope

The verification laboratory covers core formal validation domains:

* **Out-of-Repo Theorem Proving:** Decoupled symbolic execution via Microsoft's Z3 SMT solver, ensuring zero compiler overhead, zero binary distribution impact, and absolute protection of proprietary source code intellectual property.
* **Module-Specific Invariant Validation:** Bit-vector arithmetic, real-number boundary analysis, and first-order logic proofs covering all sub-systems (including cryptographic pipelines, kernel sysctl controls, memory safety, process lifecycle management, namespace isolation, and peripheral security).
* **Zero-Panic & Policy Enforcement:** Mathematical guarantees yielding unsatisfiable (`unsat`) solver results to prove that anomalous states, resource boundary violations, and unauthorized security overrides categorically trigger immediate fault exceptions.
* **Autonomous Reproducibility:** Lightweight, self-contained development sandboxing (`python3-venv`) operating entirely independently of external institutional or corporate dependencies.

---

## Licensing & Intellectual Property

All verification scripts, formal models, and documentation in this repository are the exclusive intellectual property of the author and are licensed under:

- **Author:** Viktorija Nađ  
- **License:** [CC BY-NC-ND 4.0](https://creativecommons.org/licenses/by-nc-nd/4.0/) (Creative Commons Attribution-NonCommercial-NoDerivatives 4.0 International)  

*Commercial use, derivation of operational logic, and the creation of derivative works are strictly prohibited without explicit written consent from the author.*
