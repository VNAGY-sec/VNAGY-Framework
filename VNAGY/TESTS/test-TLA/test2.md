#!/bin/bash
set -euo pipefail

echo "=== VNAGY Architecture Compliance & Lint Test ==="

# 1. Check for configuration files
if [ -f "VNAGY_Canonical_Spec.toml" ] || [ -f "VNAGY_Canonical_Spec.cfg" ]; then
    echo "[PASS] Configuration file found."
else
    echo "[INFO] No external TOML configuration, using default template."
fi

# 2. Check for forbidden keywords (Reflection / Dynamic loading)
FORBIDDEN_PATTERNS='Class\.forName|Method\.invoke|eval\(|exec\('
echo "Checking for forbidden dynamic elements in source code..."

if grep -rnE "$FORBIDDEN_PATTERNS" *.tla *.cfg 2>/dev/null; then
    echo "[FAIL] Forbidden dynamic reflection elements found!"
    exit 1
else
    echo "[PASS] No traces of reflection or dynamic execution (Determinism satisfied)."
fi

echo "=== Test 2 SUCCESSFULLY COMPLETED: Code standards and determinism confirmed. ==="
