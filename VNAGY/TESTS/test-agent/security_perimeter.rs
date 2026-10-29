use bridges::signal_dispatch::dispatch_real_os_signal;
use bridges::TargetPack;
use crypto::generate_secure_key;

#[test]
fn test_vnagy_comprehensive_perimeter_and_zero_key_audit() {
    // 1. Initialize secure master key using OS random entropy (zero hardcoding)
    let master_key = generate_secure_key().expect("Failed to generate master key");

    // 2. Define all mandatory VNAGY target packs for multi-channel perimeter verification
    let target_packs = [
        TargetPack::LPack,
        TargetPack::CPack,
        TargetPack::PPack,
    ];

    println!("\n=== VNAGY PERIMETER CRYPTOGRAPHIC AUDIT START ===");

    // 3. Exhaustive multi-channel loop audit using real OS telemetry bridge pipeline
    for target in target_packs {
        let secure_packet = dispatch_real_os_signal(&master_key, target)
            .expect("Bridge dispatch failed during multi-channel audit");

        // Invariant A: Perimeter security flag must be rigorously validated
        assert!(
            secure_packet.is_secured, 
            "CRITICAL SECURITY VIOLATION: Packet for target pack {:?} failed perimeter verification flag", 
            target
        );

        let cipher_len = secure_packet.secured_data.ciphertext.len();

        // Invariant B: Validate ciphertext is within the strict dynamic telemetry envelope (75-90 bytes)
        assert!(
            cipher_len >= 75 && cipher_len <= 90,
            "Ciphertext length {} is outside the valid perimeter envelope for {:?}",
            cipher_len,
            target
        );

        // Invariant C: Nonce length must strictly match 12 bytes
        assert_eq!(
            secure_packet.secured_data.nonce.len(),
            12,
            "Nonce length must strictly match 12 bytes for {:?}",
            target
        );

        // VISUAL PROOF FOR DOCUMENTATION / YOUTUBE / ZENODO:
        let ciphertext_hex: String = secure_packet.secured_data.ciphertext
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect();
            
        let nonce_hex: String = secure_packet.secured_data.nonce
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect();

        println!("\n[SECURED CHANNEL: {:?}]", target);
        println!(" > Status Is Secured : {}", secure_packet.is_secured);
        println!(" > Nonce (12 bytes)  : {}", nonce_hex);
        println!(" > Ciphertext ({}B)  : {}", cipher_len, ciphertext_hex);
    }

    println!("\n=== VNAGY PERIMETER CRYPTOGRAPHIC AUDIT SUCCESS ===");
}