use regex::Regex;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::Command;
use walkdir::{DirEntry, WalkDir};

// Verifica se há redirecionamentos de rede suspeitos relacionados ao frida
pub fn check_redirects() -> bool {
    let output = Command::new("sh")
        .arg("-c")
        .arg("ip route show | grep -i frida")
        .output()
        .expect("Failed to execute command");

    let result = String::from_utf8_lossy(&output.stdout);
    result.contains("frida")
}

// Função burpcheck atualizada para procurar em diretórios comuns do Android
pub fn burpcheck() -> bool {
    let paths_to_check = ["/sdcard", "/sdcard/Downloads"];

    let re = Regex::new(r"(?i)\bPortSwigger\b").expect("Invalid regex pattern");

    for start_path in paths_to_check.iter() {
        if Path::new(start_path).exists() {
            for entry in WalkDir::new(start_path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(is_cer_file)
            {
                let path = entry.path();

                if let Ok(mut file) = File::open(path) {
                    let mut contents = Vec::new();
                    if file.read_to_end(&mut contents).is_ok() {
                        let contents_str = String::from_utf8_lossy(&contents);

                        if re.is_match(&contents_str) {
                            return true;
                        }
                    }
                }
            }
        }
    }

    false
}

// Detecta se um proxy está configurado no sistema
pub fn detect_proxy() -> bool {
    let output_host = Command::new("sh")
        .arg("-c")
        .arg("settings get global http_proxy")
        .output()
        .expect("Failed to execute command");

    let output_port = Command::new("sh")
        .arg("-c")
        .arg("settings get global http_proxy_port")
        .output()
        .expect("Failed to execute command");

    let proxy_host = String::from_utf8_lossy(&output_host.stdout)
        .trim()
        .to_string();
    let proxy_port = String::from_utf8_lossy(&output_port.stdout)
        .trim()
        .to_string();

    !proxy_host.is_empty() && !proxy_port.is_empty()
}

fn is_cer_file(entry: &DirEntry) -> bool {
    entry.path().extension().and_then(|s| s.to_str()) == Some("cer")
}
