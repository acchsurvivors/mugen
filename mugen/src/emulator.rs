use crate::utilit;

pub fn is_emulator() -> bool {
    let result = 
        check_device_props() ||  
        check_qemu_drivers() || 
        check_cpu_features() || 
        check_opengl_renderer() ||
        check_filesystem_mounts();

    result
}

// verifica se o modo em que o boot foi iniciado é diferente do normal (unknown , recovery etc)
//É uma detecção um pouco agressiva demais
/* 
fn check_bootmode() -> bool{
    let bootmode = "ro.bootmode";
    let normal = "normal";
    if utilit::get_prop(&bootmode) != normal{
        return true;
    }
    else {
        return false;
    }
}*/

//verifica uma serie de variaveis que podem indicar um emulador 
fn check_device_props() -> bool {
    let props = vec![
        ("ro.kernel.qemu", "1"),
        ("ro.bootloader", "unknown"),
       //("ro.baseband","unknown"), Detecção um pouco agressiva
        ("ro.hardware", "goldfish"),
        ("ro.hardware", "ranchu"),
        ("ro.product.device", "generic"),
        ("ro.product.model", "sdk"),
        ("ro.product.model", "google_sdk"),
        ("ro.product.name", "sdk"),
        ("ro.product.name", "sdk_x86"),
        ("ro.product.name", "sdk_google"),
        ("ro.product.name", "Andy"),
        ("ro.product.name", "Droid4X"),
        ("ro.product.name", "nox"),
        ("ro.product.name", "ttVM_Hdragon"),
        ("ro.product.manufacturer", "Genymotion"),
        ("ro.product.manufacturer", "Andy"),
        ("ro.product.manufacturer", "BlueStacks"),
        ("ro.product.manufacturer", "ARC Welder"),
        ("ro.product.manufacturer", "Nox"),
        ("ro.product.model", "Emulator"),
        ("ro.product.model", "Android SDK built for x86"),
        ("ro.product.brand", "generic"),
        ("ro.product.device", "generic"),
        ("ro.product.device", "generic_x86"),
        ("ro.product.device", "generic_x86_64"),
        ("ro.product.name", "sdk"),
        ("ro.product.name", "sdk_x86"),
        ("ro.product.name", "sdk_google"),
        ("ro.build.fingerprint", "generic"),
        ("ro.build.fingerprint", "generic_x86"),
        ("ro.build.fingerprint", "generic_x86_64"),
        ("ro.build.fingerprint", "Android/sdk_google_phone_x86_64/generic_x86_64:10/QQ3A.200805.001/6578215:userdebug/dev-keys"),
        ("ro.hardware.audio.primary", "goldfish"),
        ("ro.hardware.audio.a2dp", "goldfish"),
        ("ro.hardware.audio.usb", "goldfish"),
        ("ro.hardware.audio.r_submix", "goldfish"),
        ("ro.hardware.audio.bluetooth", "goldfish"),
        ("ro.hardware.audio.legacy", "goldfish"),
        ("ro.hardware.graphics", "goldfish"),
        ("ro.hardware.graphics.egl", "goldfish"),
        ("ro.hardware.graphics.allocator", "goldfish"),
        ("init.svc.vbox86-setup", "running"),
        ("init.svc.qemud", "running"),
        ("init.svc.qemu-props", "running"),
        ("init.svc.goldfish-logcat", "running"),
        ("init.svc.goldfish-setup", "running"),
    ];

    for (prop, value) in props {
        if utilit::get_prop(prop) == value {
            return true;
        }
    }

    false
}



//Alguns checks que só funcionam em celulares antigos ou com permissionamento antigo 
fn check_opengl_renderer() -> bool {
    if let Ok(content) = std::fs::read_to_string("/proc/dri/0/name") {
        if content.contains("vbox") || content.contains("qemu") {
            return true;
        }
    }
    false
}

//só funciona em androids velhos
fn check_filesystem_mounts() -> bool {
    if let Ok(content) = std::fs::read_to_string("/proc/mounts") {
        if content.contains("vboxsf") {
            return true;
        }
    }
    false
}

//só funciona em androids velhos
fn check_qemu_drivers() -> bool {
    let drivers = vec![
        "/proc/tty/drivers",
        "/proc/cpuinfo",
    ];

    for driver in drivers {
        if let Ok(content) = std::fs::read_to_string(driver) {
            if content.contains("goldfish") || content.contains("qemu") || content.contains("vboxguest") || content.contains("vboxuser") {
                return true;
            }
        }
    }

    false
}

//só funciona em androids velhos
fn check_cpu_features() -> bool {
    if let Ok(content) = std::fs::read_to_string("/proc/cpuinfo") {
        let lower_content = content.to_lowercase();
        if lower_content.contains("qemu") || lower_content.contains("virtualbox") ||
           lower_content.contains("vmware") || lower_content.contains("kvm") ||
           lower_content.contains("xen") || lower_content.contains("hyper-v") ||
           lower_content.contains("hypervisor") {
            return true;
        }
    }
    false
}

