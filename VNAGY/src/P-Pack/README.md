# P-Pack

Purpose:
Generates symbolic pattern structures from local event sequences. 
P-Pack does not infer behavior; it only produces PatternStream units.

Inputs:
- Local sequence signals (integration/)
- Static configuration (config/ppack.toml)

Outputs:
- PatternStream (symbolic, deterministic)

Constraints:
- No dynamic pattern learning
- No probabilistic models
- No global mutable state
- Offline-first execution only

Structure:
- config/        # Static TOML definitions
- integration/   # Local OS adapters
- src/           # Deterministic symbolic logic
- tests/         # Deterministic test suite

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


