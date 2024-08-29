struct Report {
    PID: i32,
    ProcessName: String,
    UserName: String, 
    CPU: f32,
    Memory: f32,
    Command: String,
}

fn main() {
    let dir = std::fs::read_dir("/proc").unwrap();
    for entry in dir {
        let entry = entry.unwrap();
        let path = entry.path();
        let path_str = path.to_str().unwrap();
        if path.is_dir() && path_str.chars().all(char::is_numeric) {
            let pid = path.file_name().unwrap().to_str().unwrap();
            let status = std::fs::read_to_string(path.join("status")).unwrap();
            let status = status.split('\n');
            let mut report = Report {
                PID: pid,
                ProcessName: "",
                UserName: "",
                CPU: "",
                Memory: "",
                Command: "",
            };
            for line in status {
                let mut parts = line.split(':');
                let key = parts.next().unwrap();
                let value = parts.next().unwrap();
                match key {
                    "Name" => report.ProcessName = value.trim(),
                    "Uid" => {
                        let uid = value.trim().split_whitespace().next().unwrap();
                        let passwd = std::fs::read_to_string("/etc/passwd").unwrap();
                        let passwd = passwd.split('\n');
                        for line in passwd {
                            let mut parts = line.split(':');
                            let name = parts.next().unwrap();
                            let _ = parts.next();
                            let id = parts.next().unwrap();
                            if id == uid {
                                report.UserName = name;
                                break;
                            }
                        }
                    }
                    "VmRSS" => report.Memory = value.trim(),
                    "Cpus_allowed" => report.CPU = value.trim(),
                    _ => {}
                }
            }
            let cmd = std::fs::read_to_string(path.join("cmdline")).unwrap();
            report.Command = cmd.trim().replace('\0', " ");
            println!("{:?}", report);
        }
    }
}
