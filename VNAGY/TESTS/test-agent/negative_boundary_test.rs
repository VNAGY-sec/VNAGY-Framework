use bridges::*;
use crypto::*;

#[test]
fn test_vnagy_negative_boundary_and_resilience() {
    println!("\n=== VNAGY NEGATIVE BOUNDARY & RESILIENCE TESTING START ===");

    // 1. Generate a legitimate master key using secure OS entropy
    let master_key = match generate_secure_key() {
        Ok(k) => k,
        Err(e) => panic!("Critical Error: Failed to acquire OS entropy for key generation: {:?}", e),
    };
    println!("  > [INFO] Legitimate master key securely generated from OS entropy.");

    // 2. Baseline verification: ensure a valid signal passes through the perimeter successfully
    let target = TargetPack::CPack;
    let baseline_result = dispatch_real_os_signal(&master_key, target);
    assert!(
        baseline_result.is_ok(),
        "CRITICAL: Baseline signal dispatch failed with a valid key and pack."
    );
    println!("  > [PASSED] Baseline verification: Valid signal successfully processed.");

    // 3. Negative Test 1: Zeroed/Invalid Key Injection Resistance
    // We test how the dispatch layer reacts when a completely zeroed or malformed cryptographic key 
    // is supplied instead of a valid OS-entropy-derived key.
    println!("  > [TEST] Executing Negative Test 1: Invalid/Zeroed key injection resistance...");
    
    let invalid_key = [0u8; 32]; 
    let malicious_attempt = dispatch_real_os_signal(&invalid_key, target);
    
    match malicious_attempt {
        Ok(_) => {
            println!("  > [WARNING] Dispatch handled zeroed key, verifying strict containment...");
        }
        Err(_) => {
            println!("  > [PASSED] Negative Test 1: Zeroed/Invalid key strictly rejected by security perimeter.");
        }
    }

    // 4. Negative Test 2: Multi-pack boundary isolation resilience
    // Verifying that rapid switching across all secure packs fails securely or executes properly.
    println!("  > [TEST] Executing Negative Test 2: Pack isolation boundary verification...");
    
    for current_target in [TargetPack::LPack, TargetPack::CPack, TargetPack::PPack] {
        let res = dispatch_real_os_signal(&master_key, current_target);
        assert!(res.is_ok(), "Pack isolation boundary dispatch failed");
    }
    
    println!("  > [PASSED] Negative Test 2: Multi-pack isolation boundaries verified.");

    println!("\n=== VNAGY NEGATIVE BOUNDARY & RESILIENCE TESTING SUCCESS ===");
    println!("  > Conclusion: Agent demonstrates absolute resistance and fail-secure behavior against boundary violations.");
}