# VNAGY-EDGE: Yocto Project & QEMU Integration Test Suite

Central repository for embedded verification, system image generation, and QEMU-based emulation testing of the VNAGY-EDGE architectural extension[cite: 4].

## Purpose

This directory contains build configurations, Yocto layer metadata, and hardware emulation test harnesses dedicated to systematically evaluating the agent within minimal, resource-constrained embedded environments[cite: 4].

## Test Scope

The test suite covers four core verification domains for embedded deployments[cite: 4]:

- **Kernel & Signal Ingestion Layer Verification:** Testing low-level event capturing (CPU, memory, process, network) and cryptographically secured AEAD transit across pack channels (LPack, CPack, PPack)[cite: 7].

- **Yocto Image Optimization & Isolation:** Validation of minimal Linux OS builds stripped of non-essential packages, maintaining strict operational isolation and zero garbage collection runtime dependencies[cite: 4, 7].

- **Empirical Memory & Resource Profiling:** Continuous monitoring of core resident set size (RSS ~2.5 MB) and verifying system boundary constraints (< 50 MB total footprint ceiling) under Valgrind and AddressSanitizer (ASan)[cite: 4, 7, 8].

- **QEMU Emulation & Fault Resilience:** Deterministic stress testing inside QEMU virtualized instances, including resource starvation, out-of-memory (OOM) handling, watchdog recovery, and offline-first network partitioning resilience[cite: 5, 7, 8].

---

## Licensing & Intellectual Property

All build recipes, test scenarios, and architectural models in this directory are the exclusive intellectual property of the author and are licensed under[cite: 11]:

- **Author:** Viktorija Nađ[cite: 1]

- **License:** [VNAGY CC BY-NC-ND 4.0](https://creativecommons.org/licenses/by-nc-nd/4.0/) (Creative Commons Attribution-NonCommercial-NoDerivatives 4.0 International)[cite: 1, 11]


*Commercial use, operational reconstruction, and the creation of derivative works are strictly prohibited without explicit written consent from the author[cite: 11].*
