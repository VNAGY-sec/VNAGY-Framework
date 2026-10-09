# VNAGY Test Suite

Central repository for verification, validation, and security testing of the VNAGY framework.

## Purpose

This directory contains test suites, formal specifications, and verification tooling dedicated to systematically evaluating system properties and operational boundaries.

## Test Scope

The test suite covers four core verification domains:

- **Architecture Verification:** Evaluation of structural isolation, elimination of global mutable state, and compliance with normative rules.
- **Logic Verification:** Formal state machine verification, state transition correctness, and temporal invariant safety using TLA+ and model checking.
- **Long-Term System Stability:** Resource footprint profiling, fault injection resistance, and system behavior under extended operational execution.
- **Behavioral Anomaly Detection:** Execution pattern analysis and anomaly identification using a behavioral detection approach via the VNAGY prototype.

---

## Licensing & Intellectual Property

All test scenarios, scripts, and formal models in this directory are the exclusive intellectual property of the author and are licensed under [cite: 45, 47]:

- **Author:** Viktorija Nađ[cite: 25, 45]
- **License:** [CC BY-NC-ND 4.0](https://creativecommons.org/licenses/by-nc-nd/4.0/) (Creative Commons Attribution-NonCommercial-NoDerivatives 4.0 International)

*Commercial use and the creation of derivative works are strictly prohibited without explicit written consent from the author.*
