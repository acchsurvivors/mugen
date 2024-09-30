use regex::Regex;
use std::path::Path;
use std::process::Command;
use walkdir::WalkDir;

pub fn check_suspicious_files() -> bool {
    let paths_to_check = ["/sdcard", "/sdcard/Downloads"];
    let re = Regex::new(r"(?i)frida").expect("Invalid regex pattern");

    for start_path in paths_to_check.iter() {
        if Path::new(start_path).exists() {
            for entry in WalkDir::new(start_path).into_iter().filter_map(|e| e.ok()) {
                let path_str = entry.path().to_string_lossy().to_string();
                if re.is_match(&path_str) {
                    return true;
                }
            }
        }
    }

    false
}

// Verifica se há processos relacionados ao frida em execução
pub fn check_frida_process() -> bool {
    let output = Command::new("sh")
        .arg("-c")
        .arg("ps | grep -i frida")
        .output()
        .expect("Failed to execute command");

    let result = String::from_utf8_lossy(&output.stdout);
    result.contains("frida")
}


// Verifica se a porta 27042 (padrão do frida) está em uso
pub fn check_frida_port() -> bool {
    let output = Command::new("sh")
        .arg("-c")
        .arg("netstat -an | grep 27042")
        .output()
        .expect("Failed to execute command");

    let result = String::from_utf8_lossy(&output.stdout);
    result.contains("27042")
}
