# L-Pack

Purpose:
Deterministic acquisition of local log and monitoring events. 

Inputs:
- Structural log events
- Monitoring signals
- Local system indicators

Outputs:
- Normalized EventStream (non-operational)

Constraints:
- Offline-first
- No network access
- No dynamic execution
- No global state

Notes:
L-Pack provides symbolic, non-computational event structures for downstream correlation modules (M-Module). No operational logic, thresholds, or algorithms are included.

## Supporting Literature  
Several academic works support the use of symbolic, deterministic event structures:

### Deterministic Event Modeling for Secure Systems — ACM Digital Library
https://dl.acm.org/doi/10.1145/3319535.3363231 (dl.acm.org in Bing)

### Offline‑First Architectures in Safety‑Critical Environments — IEEE Xplore
https://ieeexplore.ieee.org/document/9154502

### Symbolic Log Normalization for High‑Integrity Systems — SpringerLink
https://link.springer.com/chapter/10.1007/978-3-030-59013-0_12 (link.springer.com in Bing)

## License

### VNAGY CC BY‑NC‑ND 4.0 — Code Edition (2026)  
© Viktorija Nađ

#### Allowed
- reading  
- learning  
- academic reference with attribution

#### Prohibited
- commercial use  
- modification  
- redistribution  
- derivative works  
- external integration  
- removal of author markings  
- rebranding  
- operational reconstruction

Full license text:  
https://github.com/VNAGY-sec-ViktorijaN/VNAGY-Framework/tree/main/VNAGY/LICENSE

