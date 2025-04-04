use sysinfo::{CpuExt, System, SystemExt};
use whoami::fallible;

use crate::uptime;

pub fn system_info(system: &System) {
    let username = whoami::username();
    let hostname = fallible::hostname().unwrap_or_else(|_| "Unknown".to_string());
    let os_name = system.name().unwrap_or_else(|| "Unknown".to_string());
    let kernel_version = system
        .kernel_version()
        .unwrap_or_else(|| "Unknown".to_string());
    let uptime = uptime::format_uptime(system.uptime());
    let cpu_info = system
        .cpus()
        .first()
        .map_or("Unknown".to_string(), |cpu| cpu.brand().to_string());
    let memory_used = system.used_memory() / 1024 / 1024; // Convert to GB
    let memory_total = system.total_memory() / 1024 / 1024;

    println!("User: {}@{}", username, hostname);
    println!("---------------------------------------");
    println!("OS: {}", os_name);
    println!("Kernel: {}", kernel_version);
    println!("Uptime: {}", uptime);
    println!("CPU: {}", cpu_info);
    println!("Memory: {} / {} MB", memory_used, memory_total);
}
