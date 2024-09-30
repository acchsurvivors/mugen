use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use crate::utilit;


struct Property {
    name: &'static str,
    sus: Option<&'static str>,
}

const KNOWN_ROOT_PROPERTIES: [Property; 3] = [
    Property {
        name: "ro.secure",
        sus: Some("0"),
    },
    Property {
        name: "ro.debuggable",
        sus: Some("1"),
    },
    Property {
        name: "ro.build.type",
        sus: Some("userdebug"),
    },
];

//verifica alfumas propriedades que podem indicar um ambiente inseguro
pub fn check_known_root_properties() -> bool {
    KNOWN_ROOT_PROPERTIES.iter().any(|prop| {
        let value = utilit::get_prop(prop.name);  
        prop.sus.map_or(false, |sus| value == sus) 
    })
}


//tenta criar arquivos em diretorios que não deveria ter permissão
pub fn check_writable_paths() -> bool {
    let non_writable_paths = [
        "/system",
        "/system/bin",
        "/system/sbin",
        "/system/xbin",
        "/vendor/bin",
        "/sbin",
        "/etc",
    ];

    non_writable_paths
        .iter()
        .any(|&path| is_directory_writable(path))
}

fn is_directory_writable(directory_path: &str) -> bool {
    let path = Path::new(directory_path);
    if !path.exists() || !path.is_dir() {
        return false;
    }

    let test_file = path.join("test.tmp");
    match fs::File::create(&test_file) {
        Ok(_) => {
            let _ = fs::remove_file(test_file);
            true
        }
        Err(_) => false,
    }
}

//verifica a key
pub fn check_test_keys() -> bool {
    if let Ok(build_tags) = std::env::var("ro.build.tags") {
        return build_tags.contains("test-keys");
    }
    false
}

//tenta executar o comando su
pub fn is_device_rooted_exec() -> bool {
    match std::process::Command::new("su").output() {
        Ok(_) => true,
        Err(_) => false,
    }
}

//procura nas variaveis de ambiente pelo su ou algo que possa indicar o magisk
pub fn check_root_path() -> bool {
    if let Ok(path_env) = std::env::var("PATH") {
        for path in path_env.split(':') {
            let su_file = Path::new(path).join("su");
            if su_file.exists() {
                return true;
            }
            let magisk_file = Path::new(path).join("magisk");
            if magisk_file.exists() {
                return true;
            }
        }
    }
    false
}

pub fn check_exec_which_su() -> bool {
    match std::process::Command::new("which").arg("su").output() {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

//em versões mais novas do android(Android 10 ou superior) não teremos acesso ao arquivo e só retornarermos false essa verificação é destinada a versões anteriores ao android 10
pub fn is_blacklisted_mount_path(mount_info_path: &str) -> bool {
    let blacklisted_paths = [
        "/sbin/.magisk/",
        "/data/magisk/",
        "/cache/magisk/",
        "/system/.magisk/",
        "/system/xbin/su",
        "/system/bin/su",
        "magisk",
        "core/mirror",
        "core/img",
    ];

    if let Ok(file) = File::open(mount_info_path) {
        let reader = io::BufReader::new(file);

        // Lê cada linha do arquivo
        for line in reader.lines() {
            if let Ok(line) = line {
                // Checa se algum dos caminhos na lista negra está presente na linha
                for blacklisted in blacklisted_paths {
                    if line.contains(blacklisted) {
                        return true;
                    }
                }
            }
        }
    }
    // Se não conseguiu ler o arquivo ou não encontrou nada, retorna falso
    false
}
const SU_FILES: [&str; 65] = [
    "/sbin/su",
    "/system/bin/su",
    "/system/xbin/su",
    "/data/local/xbin/su",
    "/data/local/bin/su",
    "/system/sd/xbin/su",
    "/system/bin/failsafe/su",
    "/data/local/su",
    "/su/bin/su",
    "/system/bin/.ext/su",
    "/system/usr/we-need-root/su",
    "/cache/su",
    "/dev/su",
    "/data/su",
    "/su",
    "/system/app/Superuser.apk",
    "/system/app/SuperSU.apk",
    "/system/app/SuperSU",
    "/system/app/SuperSU/SuperSU.apk",
    "/system/app/Kinguser.apk",
    "/system/app/KingUser.apk",
    "/system/lib/libsu.so",
    "/system/lib64/libsu.so",
    "/data/data/com.noshufou.android.su/",
    "/data/data/eu.chainfire.supersu/",
    "/system/xbin/daemonsu",
    "/system/xbin/busybox",
    "/data/media/0/TWRP",
    "/sdcard/TWRP",
    "/data/TWRP",
    "./frida-server",
    "/data/local/tmp/frida-server",
    "/system/framework/root-access.jar",
    "/system/su.d",
    "/system/xbin/ku.sud",
    "/system/xbin/daemonsu",
    "/system/xbin/supolicy",
    "/system/xbin/supolicy.so",
    "/system/xbin/resize2fs_static",
    "/system/xbin/sush",
    "/system/xbin/busybox",
    "/system/xbin/busybox_mksh",
    "/system/xbin/busybox_insmod",
    "/system/xbin/busybox_rmmod",
    "/system/xbin/toybox",
    "/data/local/tmp/su",
    "/data/local/tmp/supolicy",
    "/data/local/tmp/busybox",
    "/data/local/tmp/magisk",
    "/data/local/tmp/frida-server",
    "/data/local/tmp/frida64",
    "/data/local/tmp/magiskhide",
    "/data/local/tmp/magiskcore",
    "/data/adb/ksu",
    "/system/lib/libc_malloc_debug_qemu.so",
    "/system/bin/qemud",
    "/sys/qemu_trace",
    "/system/bin/androVM-prop",
    "/system/bin/microvirt-prop",
    "/dev/vboxguest",
    "/dev/vboxuser",
    "/mnt/prebundledapps/",
    "/system/bluestacks.prop",
    "/system/bin/qemu-props",
    "/sys/devices/virtual/misc/qemu_pipe",
];

fn detect_su_files() -> bool {
    SU_FILES.iter().any(|&file| Path::new(file).exists())
}

pub fn is_root() -> bool {
    let result = 
    check_known_root_properties() ||  
    check_writable_paths() || 
    check_test_keys() || 
    is_device_rooted_exec() ||
    check_root_path() ||
    check_exec_which_su() ||
    detect_su_files() ||
    is_blacklisted_mount_path("/proc/self/mountinfo");

    result
}