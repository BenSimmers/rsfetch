mod ascii;
mod system_info;
mod uptime;
use sysinfo::{System, SystemExt};

fn main() {
    print!("\x1B[2J\x1B[1;1H");
    let mut system = System::new_all();
    system.refresh_all();

    // Determine ASCII art based on OS
    match system.name() {
        Some(name) => {
            if name.contains("Linux") {
                ascii::linux_ascii();
            } else if name.contains("Windows") {
                ascii::windows_ascii();
            } else if name.contains("Darwin") || name.contains("macOS") {
                ascii::macos_ascii();
            } else {
                println!("sysfetch");
            }
        }
        None => {
            println!("sysfetch");
        }
    }
    system_info::system_info(&system);
}
