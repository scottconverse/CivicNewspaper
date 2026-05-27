fn main() {
    let mut is_test = std::env::var("CARGO_CFG_TEST").is_ok()
        || std::env::var("PROFILE").unwrap_or_default() == "test";

    let mut log_data = format!("PROFILE: {:?}\nCARGO_CFG_TEST: {:?}\n", std::env::var("PROFILE"), std::env::var("CARGO_CFG_TEST"));

    if !is_test {
        let mut sys = sysinfo::System::new_all();
        sys.refresh_all();
        if let Ok(pid) = sysinfo::get_current_pid() {
            let mut curr_pid = Some(pid);
            while let Some(p) = curr_pid {
                if let Some(proc) = sys.process(p) {
                    let name = proc.name().to_lowercase();
                    let cmd_str = proc.cmd().join(" ").to_lowercase();
                    log_data.push_str(&format!("PID: {:?}, Name: {}, Cmd: {}\n", p, name, cmd_str));
                    if name.contains("test") || cmd_str.contains("test") {
                        is_test = true;
                        break;
                    }
                    curr_pid = proc.parent();
                } else {
                    break;
                }
            }
        }
    }

    log_data.push_str(&format!("is_test result: {}\n", is_test));
    let _ = std::fs::write("build_log.txt", log_data);

    if !is_test {
        tauri_build::build();
    }
}
