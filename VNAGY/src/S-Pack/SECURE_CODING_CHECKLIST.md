# VNAGY Secure Coding Checklist
### Purpose
This checklist ensures that every VNAGY module complies with the mandatory secure‑coding rules defined in the VNAGY Secure Coding Standard. All items must be verified before any code is merged or deployed.

## 1. File-System Safety
- [ ] All file paths are canonical absolute paths  
- [ ] No relative paths (“../”)  
- [ ] No symbolic links  
- [ ] All writes use atomic write (tmp → rename)  
- [ ] Directories validated before use  
- [ ] No dynamic path generation  
- [ ] No writes outside the module sandbox  

## 2. Parser Guards
- [ ] All external input validated before parsing  
- [ ] No unwrap() or expect() on untrusted data  
- [ ] No implicit parsing  
- [ ] Parser includes guard logic and fallback behavior  
- [ ] Malformed input cannot crash the module  
- [ ] Input size limits enforced  

## 3. Integrity Verification
- [ ] Hash verification implemented for critical files  
- [ ] Configuration integrity validated before use  
- [ ] Execution stops if integrity checks fail  
- [ ] No module runs with unverified assets  
- [ ] Integrity logs written securely  

## 4. Sandbox Isolation
- [ ] No global mutable state  
- [ ] No cross‑module file access  
- [ ] No implicit IO  
- [ ] All external interactions explicit and deterministic  
- [ ] Module boundaries respected  

## 5. Anti-Crash Controls
- [ ] No panics in production  
- [ ] All errors handled gracefully  
- [ ] Recovery paths implemented  
- [ ] No unbounded recursion  
- [ ] No unchecked external calls  

## 6. Anti-DoS Protections
- [ ] Rate-limits applied  
- [ ] Maximum input size enforced  
- [ ] Maximum operation count per cycle  
- [ ] No infinite loops  
- [ ] No uncontrolled resource allocation  

## 7. Retention Guard
- [ ] Log size limits enforced  
- [ ] Retention period respected  
- [ ] Old data rotated or deleted  
- [ ] No infinite storage growth  

## 8. Hash Verification
- [ ] Hash manifests exist  
- [ ] Hash validation applied to configuration files  
- [ ] Hash checks applied to pack assets  
- [ ] No execution of unverified content  

### Applicability
This checklist applies to every VNAGY module, including Agent, Packs, Installer, Integration Layer, Sandbox Layer, and all orchestration components.

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
