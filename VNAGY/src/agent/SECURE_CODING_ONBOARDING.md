# VNAGY Secure Coding — Team Onboarding Guide
### Purpose
This guide introduces new contributors to the mandatory secure‑coding principles used across all VNAGY modules. It provides a clear and accessible overview of the security practices required for deterministic, safe, and offline‑first development.

## 1. Safe File Handling
VNAGY treats the file system as hostile. Use absolute canonical paths, reject symbolic links, forbid “../”, validate directories, and write files using atomic write (tmp → rename). Never generate dynamic paths or write outside the module sandbox.

## 2. Safe Input Parsing
All external input is untrusted. Validate before parsing, avoid unwrap() and expect(), enforce size limits, reject malformed input safely, and ensure every parser has guard logic and fallback behavior.

## 3. Integrity Checks
Modules must verify the authenticity of all critical files and configurations. Validate hashes, stop execution on integrity failure, and never run with modified or unverified assets.

## 4. Sandbox Isolation
Modules operate in strict isolation. No global mutable state, no cross‑module file access, no implicit IO, and all external interactions must be explicit and deterministic.

## 5. Crash Resistance
Modules must not crash. Handle errors gracefully, avoid panics, implement recovery paths, and prevent unbounded recursion or unchecked external calls.

## 6. Anti-DoS Protection
Modules must resist overload. Apply rate‑limits, enforce maximum input size, limit operations per cycle, and avoid infinite loops or uncontrolled resource allocation.

## 7. Retention Guard
Storage must be controlled. Enforce log size limits, retention periods, safe rotation or deletion of old data, and prevent infinite growth.

## 8. Hash Verification
Critical inputs must be cryptographically validated. Maintain hash manifests, validate configuration and pack asset hashes, and reject unverified content.

### Developer Responsibility
Every VNAGY developer must follow these rules, use the secure‑coding checklist during review, and treat security as an integral part of normal development.

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
