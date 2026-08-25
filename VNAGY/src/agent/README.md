# agent

Purpose:
Deterministic offline-first runtime component responsible for executing local VNAGY modules on the endpoint.

Inputs:
- Static configuration (config/)
- OS-level integration signals (integration/)
- Local module definitions (src/)

Outputs:
- Explicit module state transitions
- Deterministic EventStream structures
- Local-only diagnostic artifacts (tests/)

Constraints:
- No network access
- No dynamic execution
- No global mutable state
- All behavior must be deterministic under identical inputs

Structure:
- config/        # Agent configuration blocks
- integration/   # OS-level integration points
- src/           # Agent runtime logic (non-operational)
- tests/         # Deterministic test suite

Notes:
The agent operates exclusively in offline-first mode and provides no operational detection logic. All constructs are symbolic and non-reconstructable.

VNAGY CC BY‑NC‑ND 4.0 — Code Edition (2026)

© Viktorija Nađ
Allowed

* reading
* learning
* academic reference with attribution

Prohibited

* commercial use
* modification
* redistribution
* derivative works
* external integration
* removal of author markings
* rebranding
* operational reconstruction

Full license text:
https://github.com/VNAGY-sec-ViktorijaN/VNAGY-Framework/tree/main/VNAGY/LICENSE
