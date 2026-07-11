# SoulVex Shield

A hybrid security system for network nodes. SoulVex Shield pairs high-speed packet filtering (eBPF/XDP) with low-latency hardware control (FPGA/Verilog) and a Rust-based management layer.

## Architecture
- **eBPF (XDP):** Kernel-level packet filtering.
- **Rust Daemon:** User-space controller managing state and hardware interaction.
- **FPGA Logic:** Real-time signal monitoring and actuator control.

## Quick Start

### Software (eBPF & Rust)
```bash
cargo build --manifest-path userspace/Cargo.toml --release
```

### Hardware Simulation
```bash
iverilog -I firmware -o firmware/sim.out firmware/top_module_tb.v
vvp firmware/sim.out
gtkwave firmware/simulation.vcd
```

## Project Structure
- `e_progs/`: XDP filter source.
- `userspace/`: Rust management daemon.
- `firmware/`: FPGA logic.
- `firmware/top_module_tb.v`: Testbench.
