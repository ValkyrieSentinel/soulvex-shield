# SoulVex Shield (Cybernetic Hardware Firewall)

A full-stack cyber-physical defense system combining eBPF/XDP network filtering, FPGA-based glitch detection, and a physical servo-driven shield mechanism.

## Overview
SoulVex Shield is a hardware/software co-designed security system. When network attacks are detected (via eBPF packet counters or FPGA IRQ), the system activates drop mode and physically closes a servo-driven shield. During normal operation, the servo scans 0-180 degrees for physical monitoring.

## Architecture Layers

### 1. Network Layer (eBPF/XDP)
- Kernel-space XDP program counts incoming packets via Array map
- User-space orchestrator reads counters and triggers mitigation

### 2. Hardware Layer (FPGA/Verilog)
- hardware_firewall: Detects network line glitches (2M clock cycle threshold), generates IRQ
- kinematic_servo: PWM controller for servo motor (0-180 degree positioning)

### 3. Physical Layer (Servo Motor)
- Normal mode: Servo scans 0-180 degrees (physical monitoring)
- Attack mode: Servo returns to 0 degrees (shield closes)

### 4. Orchestration Layer (Rust)
- Reads eBPF map counters
- Reads FPGA IRQ via sysfs GPIO
- Controls servo via sysfs PWM
- Tokio async runtime for real-time response

## Key Features
- Multi-layer defense: network + hardware + physical
- Real-time FPGA interrupt detection
- PWM servo control via Linux sysfs
- Lock-free eBPF map communication
- Async Rust orchestrator with graceful shutdown

## Build and Run

    cargo build --release
    sudo ./target/release/soulvex-shield --iface eth0

## Verilog Simulation

    iverilog -o sim top_module.v top_module_tb.v
    vvp sim
    gtkwave simulation.vcd

## License
MIT. Part of the ValkyrieSentinel ecosystem.
