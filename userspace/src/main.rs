use aya::maps::Array;
use aya::programs::{Xdp, XdpFlags};
use aya::Bpf;
use clap::Parser;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Parser, Debug)]
#[command(author = "Helga | SoulVex", version = "1.0", about = "SoulVex Cybernetic Hardware Controller")]
struct Args {
    #[arg(short, long, default_value = "eth0")]
    iface: String,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    let mut bpf = Bpf::load_file("target/bpfel-unknown-none/release/soulvex_firewall")?;
    
    let program: &mut Xdp = bpf
        .program_mut("soulvex_shield_xdp")
        .ok_or(anyhow::anyhow!("BPF program not found"))?
        .try_into()?;
        
    program.load()?;
    let _link = program.attach(&args.iface, XdpFlags::default())?;

    let mut cyber_map: Array<_, u32> = Array::try_from(bpf.map_mut("CYBERNETIC_MAP")?)?;
    
    init_hardware_interfaces();

    let mut current_angle: u8 = 90;
    let mut scan_direction = true;
    let mut attack_mode_active = false;

    tokio::select! {
        _ = async {
            loop {
                let packet_count: u32 = cyber_map.get(&1, 0).unwrap_or(0);
                let fpga_irq_detected = read_fpga_irq_raw(); 

                if fpga_irq_detected || packet_count > 5000 {
                    if !attack_mode_active {
                        eprintln!("[ALERT] Mitigation active. Drop mode engaged.");
                        attack_mode_active = true;
                    }
                    let _ = cyber_map.set(0, 1, 0); 
                    current_angle = 0;
                    write_fpga_kinematics_raw(current_angle);
                } else {
                    if attack_mode_active {
                        eprintln!("[INFO] System nominal. Filtering disengaged.");
                        attack_mode_active = false;
                    }
                    let _ = cyber_map.set(0, 0, 0); 
                    
                    if scan_direction {
                        current_angle = current_angle.saturating_add(5);
                        if current_angle >= 180 { scan_direction = false; }
                    } else {
                        current_angle = current_angle.saturating_sub(5);
                        if current_angle <= 0 { scan_direction = true; }
                    }
                    
                    write_fpga_kinematics_raw(current_angle);
                }

                let _ = cyber_map.set(1, 0, 0);
                sleep(Duration::from_millis(100)).await;
            }
        } => {}
        
        _ = tokio::signal::ctrl_c() => {}
    }

    Ok(())
}

fn init_hardware_interfaces() {
    if !Path::new("/sys/class/gpio/gpio25").exists() {
        let _ = fs::write("/sys/class/gpio/export", "25");
        let _ = fs::write("/sys/class/gpio/gpio25/direction", "in");
    }
    if !Path::new("/sys/class/pwm/pwmchip0/pwm0").exists() {
        let _ = fs::write("/sys/class/pwm/pwmchip0/export", "0");
        let _ = fs::write("/sys/class/pwm/pwmchip0/pwm0/period", "20000000");
        let _ = fs::write("/sys/class/pwm/pwmchip0/pwm0/enable", "1");
    }
}

fn read_fpga_irq_raw() -> bool {
    if let Ok(value) = fs::read_to_string("/sys/class/gpio/gpio25/value") {
        return value.trim() == "1";
    }
    false
}

fn write_fpga_kinematics_raw(angle: u8) {
    let pulse_ns = 1_000_000 + (angle as u32 * 5555);
    if let Ok(mut file) = fs::OpenOptions::new().write(true).open("/sys/class/pwm/pwmchip0/pwm0/duty_cycle") {
        let _ = writeln!(file, "{}", pulse_ns);
    }
}