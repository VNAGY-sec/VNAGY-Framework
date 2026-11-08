use resiliency::watchdog::HardwareWatchdog;

#[test]
fn test_hardware_watchdog_missing_device() {
    println!("\n==================================================");
    println!("[VNAGY RESILIENCY SUITE] Initializing Hardware Watchdog Test...");
    println!("--------------------------------------------------");

    let invalid_path = "/nonexistent/device/watchdog";
    println!("[INFO] Simulating edge environment without physical device at: {}", invalid_path);
    
    let result = HardwareWatchdog::new(invalid_path);

    assert!(result.is_err(), "Watchdog initialization must fail on a non-existent device path");
    
    match result {
        Err(resiliency::errors::ResiliencyError::IoError(e)) => {
            println!("[SUCCESS] Expected I/O Error safely intercepted: {}", e);
            println!("[SECURITY] Fortress trust boundary verified: No panics, clean Result propagation.");
        }
        other => panic!("Expected IoError, got {:?}", other),
    }

    println!("--------------------------------------------------");
    println!("[RESULT] Hardware Watchdog Test Passed Successfully.");
    println!("==================================================\n");
}