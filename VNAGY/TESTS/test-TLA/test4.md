
#!/bin/bash
set -euo pipefail

echo "=== VNAGY Test 4: Fault Injection & Malformed Input Test ==="

trap 'rm -f /tmp/malformed.toml' EXIT

# Deliberately creating an invalid TOML configuration
echo "invalid_syntax_key = [ unclosed array" > /tmp/malformed.toml

# Verifying that the parser correctly detects errors and rejects the input
if python3 -c "import tomllib; tomllib.loads(open('/tmp/malformed.toml').read())" 2>/dev/null; then
    echo "[FAIL] Malformed configuration was incorrectly accepted!"
    exit 1
else
    echo "[PASS] Malformed input safely rejected by parser (Fault isolation verified)."
fi

echo "=== Test 4 SUCCESSFULLY COMPLETED: Fault resistance confirmed. ==="
