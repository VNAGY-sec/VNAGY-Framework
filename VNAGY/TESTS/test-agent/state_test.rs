use std::fs;
use resiliency::errors::ResiliencyError;
use resiliency::state::ResiliencyState;
use resiliency::types::{ResiliencyPolicy, BufferedTelemetryItem};

#[test]
fn test_resiliency_state_workflow() {
    println!("\n==================================================");
    println!("[VNAGY RESILIENCY SUITE] Initializing Store-and-Forward Test...");
    println!("--------------------------------------------------");

    let temp_dir = std::env::temp_dir().join("vnagy_test_buffer");
    if temp_dir.exists() {
        let _ = fs::remove_dir_all(&temp_dir);
        println!("[SETUP] Cleaned up previous test buffer directory.");
    }

    let policy = ResiliencyPolicy {
        max_capacity_bytes: 100, // Very small limit for strict capacity testing
        enabled: true,
    };
    println!("[CONFIG] Policy set: Max capacity = {} bytes", policy.max_capacity_bytes);

    let mut state = ResiliencyState::new(temp_dir.clone(), policy).unwrap();
    println!("[SUCCESS] ResiliencyState initialized at: {:?}", temp_dir);

    // Verify initial state
    assert!(!state.is_offline());
    println!("[STATE] Agent online connectivity status verified (Online).");

    // Create a telemetry item within capacity limits
    let item = BufferedTelemetryItem {
        id: "telemetry_001".to_string(),
        payload: vec![1, 2, 3, 4, 5], // 5 bytes
    };
    println!("[TELEMETRY] Created item ID '{}' (Size: 5 bytes)", item.id);

    // Push item while offline (handled by dispatch logic or direct state call)
    state.set_offline(true);
    println!("[STATE] Network transition: Switched to OFFLINE mode.");
    
    let push_result = state.push_item(item.clone());
    assert!(push_result.is_ok());
    println!("[SUCCESS] Telemetry item successfully buffered locally.");

    // Test capacity boundary violation
    let massive_item = BufferedTelemetryItem {
        id: "telemetry_002".to_string(),
        payload: vec![0u8; 150], // Exceeds 100 bytes limit
    };
    println!("[TEST] Attempting to push massive item (150 bytes) violating 100-byte limit...");
    
    let capacity_result = state.push_item(massive_item);
    match capacity_result {
        Err(ResiliencyError::CapacityExceeded(msg)) => {
            println!("[SUCCESS] Capacity boundary enforced correctly: {}", msg);
        }
        other => panic!("Expected CapacityExceeded error, got {:?}", other),
    }

    // Pop item from buffer (simulate flushing)
    state.set_offline(false);
    println!("[STATE] Network transition: Restored to ONLINE mode.");
    println!("[FLUSH] Popping items from local offline buffer...");

    let popped = state.pop_item().unwrap();
    assert!(popped.is_some());
    let retrieved = popped.unwrap();
    println!("[SUCCESS] Retrieved buffered item ID: '{}' (Payload bytes: {:?})", retrieved.id, retrieved.payload);
    
    assert_eq!(retrieved.id, "telemetry_001");
    assert_eq!(retrieved.payload, vec![1, 2, 3, 4, 5]);

    // Cleanup
    let _ = fs::remove_dir_all(&temp_dir);
    println!("[CLEANUP] Temporary test directory safely removed.");

    println!("--------------------------------------------------");
    println!("[RESULT] Store-and-Forward Resiliency Test Passed Successfully.");
    println!("==================================================\n");
}