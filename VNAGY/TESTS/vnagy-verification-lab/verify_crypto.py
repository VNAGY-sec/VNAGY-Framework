from z3 import *

def test_z3_setup():
    """Simple test function to verify that Z3 solver is installed and working correctly."""
    print("[*] Initializing Z3 test for VNAGY verification lab...")
    
    # Create a Z3 solver instance
    s = Solver()
    
    # Define a symbolic integer variable representing a cryptographic key length
    key_length = Int('key_length')
    
    # Add a simple constraint: key length must be equal to 32 bytes (256-bit ChaCha standard)
    s.add(key_length == 32)
    
    # Check satisfiability
    result = s.check()
    
    if result == sat:
        model = s.model()
        print("[SUCCESS] Z3 solver is working properly!")
        print(f" > Test Variable (key_length) successfully resolved to: {model[key_length]} bytes")
        return True
    else:
        print("[FAIL] Z3 solver check failed.")
        return False

if __name__ == '__main__':
    test_z3_setup()
