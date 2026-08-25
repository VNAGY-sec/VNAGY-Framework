# M-Module

Purpose:
Central deterministic correlation module. 
Receives EventStream units from all Packs and produces symbolic CorrelationStream and ScoreStream outputs.

Inputs:
- EventStream from L-Pack, C-Pack, P-Pack, S-Pack, X-Pack
- Static configuration (config/mmodule.toml)

Outputs:
- CorrelationStream (deterministic)
- ScoreStream (symbolic)

Constraints:
- No dynamic correlation logic
- No probabilistic scoring
- No global mutable state
- No backflow to Packs
- Offline-first execution only

Structure:
- config/        # Static TOML definitions
- integration/   # Local OS adapters
- src/           # Deterministic correlation logic
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

