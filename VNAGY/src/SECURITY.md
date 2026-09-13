# VNAGY Security Policy (Root-Level)
### Canonical & Locked Architecture — Security Baseline

This document defines the root-level security expectations for all VNAGY modules, packs, agents, and orchestration layers.  
The VNAGY architecture is canonical, deterministic, offline-first, and structurally locked.  
Security rules in this document apply globally and uniformly across the entire framework.

---

## 1. Architectural Security Principles
VNAGY enforces strict security boundaries at the architectural level:

- Deterministic execution  
- Explicit data flow  
- No implicit IO  
- No global mutable state  
- Strict sandbox isolation  
- Verified configuration and assets  
- Offline-first operational model  

These principles are non-negotiable and apply to every module.

---

## 2. Mandatory Secure Coding Documents
Every module **must** include:

- `SECURE_CODING_ONBOARDING.md`  
- `SECURE_CODING_CHECKLIST.md`  

These documents define the normative secure coding rules and onboarding requirements for contributors.

---

## 3. Integrity & Verification Requirements
All VNAGY components must:

- Validate hashes of critical files  
- Reject unverified or modified assets  
- Abort execution on integrity failure  
- Log integrity events deterministically  

Integrity is a mandatory precondition for execution.

---

## 4. Sandbox & Isolation Guarantees
Modules operate in strict isolation:

- No cross-module file access  
- No dynamic path generation  
- No symbolic links  
- No writes outside sandbox boundaries  
- Atomic write operations only  

Isolation is enforced at design and implementation level.

---

## 5. Anti-DoS & Stability Controls
All modules must implement:

- Input size limits  
- Rate-limiting  
- Bounded operations per cycle  
- No infinite loops  
- No panics in production  

Stability is treated as a security property.

---

## 6. Retention Guard
VNAGY enforces controlled storage behavior:

- Log size limits  
- Retention periods  
- Safe rotation or deletion  
- No uncontrolled growth  

Retention is part of the global security baseline.

---

## 7. Supply-Chain Security
All dependencies must be:

- Verified  
- Version-pinned  
- Deterministic  
- Free of dynamic network fetches  

No external runtime dependency resolution is allowed.

---

## Licenses
All documents in this directory are licensed under:

**VNAGY CC BY‑NC 4.0 Extended License**  
and  
**VNAGY Minimal License (PSDL‑1.3)** for short normative documents.

Usage restrictions include:  
No reconstruction  
No operational derivation  
No commercial deployment  
No algorithmic extraction  

© Viktorija Nađ — All Rights Reserved
