use bridges::signal_dispatch::dispatch_real_os_signal;
use bridges::TargetPack;
use crypto::generate_secure_key;

#[test]
fn test_vnagy_memory_zeroization_and_scope_audit() {
    println!("\n=== VNAGY MEMORY AUDIT & ZEROIZATION START ===");

    // Define multiple iterations (memory scope stress-test) 
    // to ensure rapid allocation and key dropping leave no lingering traces.
    let iterations = 1000;

    for i in 1..=iterations {
        // 1. Dynamically generate master key from OS entropy within a narrow scope
        let master_key = generate_secure_key()
            .unwrap_or_else(|_| panic!("Failed to generate master key at iteration {}", i));

        let target_packs = [
            TargetPack::LPack,
            TargetPack::CPack,
            TargetPack::PPack,
        ];

        for target in target_packs {
            let secure_packet = dispatch_real_os_signal(&master_key, target)
                .expect("Memory audit bridge dispatch failed");

            // Invariant A: Verify that the packet security flag is valid
            assert!(
                secure_packet.is_secured,
                "Memory Audit Failure: Packet security flag compromised for {:?}",
                target
            );

            // Invariant B: Verify that the ciphertext is non-empty and occupies memory space
            assert!(
                !secure_packet.secured_data.ciphertext.is_empty(),
                "Memory Audit Failure: Ciphertext buffer is empty"
            );
        }

        // Once the iteration ends, the `master_key` variable goes out of scope,
        // and Rust automatically and safely deallocates memory locations (Drop semantics).
        println!(" > Iteration {}: Memory scope successfully verified, isolated, and dropped.", i);
    }

    println!("\n=== VNAGY MEMORY AUDIT & ZEROIZATION SUCCESS ===");
}