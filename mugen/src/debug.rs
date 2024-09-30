use jni::objects::{JClass, JObject};
use jni::sys::jboolean;
use jni::JNIEnv;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::os::raw::c_int;

extern "C" {
    fn getpid() -> c_int;
    fn kill(pid: c_int, sig: c_int) -> c_int;
}

#[no_mangle]
pub extern "C" fn try_kill() -> bool {
    unsafe {
        let pid = getpid();
        kill(pid, 0) == -1
    }
}

pub fn debug_check(env: &mut JNIEnv, _class: &JClass, context: &JObject) -> jboolean {
    let package_manager = match env.call_method(
        context,
        "getPackageManager",
        "()Landroid/content/pm/PackageManager;",
        &[],
    ) {
        Ok(pm) => pm.l().unwrap_or(JObject::null()),
        Err(_) => return 0,
    };

    if package_manager.is_null() {
        return 0;
    }

    let package_name = match env.call_method(context, "getPackageName", "()Ljava/lang/String;", &[])
    {
        Ok(pn) => pn.l().unwrap_or(JObject::null()),
        Err(_) => return 0,
    };

    if package_name.is_null() {
        return 0;
    }

    let application_info = match env.call_method(
        package_manager,
        "getApplicationInfo",
        "(Ljava/lang/String;I)Landroid/content/pm/ApplicationInfo;",
        &[(&package_name).into(), 0.into()],
    ) {
        Ok(ai) => ai.l().unwrap_or(JObject::null()),
        Err(_) => return 0,
    };

    if application_info.is_null() {
        return 0;
    }

    let flags = match env.get_field(application_info, "flags", "I") {
        Ok(field) => field.i().unwrap_or(0),
        Err(_) => return 0,
    };

    let flag_debuggable =
        match env.get_static_field("android/content/pm/ApplicationInfo", "FLAG_DEBUGGABLE", "I") {
            Ok(field) => field.i().unwrap_or(0),
            Err(_) => return 0,
        };

    (flags & flag_debuggable != 0) as jboolean
}

pub fn detect_debugger(env: &mut JNIEnv, _class: &JClass) -> jboolean {
    let debug_class = env.find_class("android/os/Debug").unwrap();

    let is_debugger_connected =
        match env.call_static_method(debug_class, "isDebuggerConnected", "()Z", &[]) {
            Ok(result) => result.z().unwrap_or(false) as jboolean,
            Err(_) => return 0,
        };

    is_debugger_connected
}

pub fn has_tracer_pid() -> bool {
    // Tenta abrir o arquivo /proc/self/status
    let file = File::open("/proc/self/status");
    if file.is_err() {
        return false; // Se houver erro ao abrir o arquivo, retorna false
    }
    let reader = BufReader::new(file.unwrap());

    // Itera sobre as linhas do arquivo
    for line in reader.lines() {
        if let Ok(line) = line {
            // Verifica se a linha começa com "TracerPid:"
            if line.starts_with("TracerPid:") {
                // Obtém o valor após "TracerPid:"
                if let Some(tracer_pid_value) = line.split_whitespace().last() {
                    // Converte o valor para inteiro e verifica se é maior que 0
                    return tracer_pid_value != "0";
                }
            }
        }
    }

    false // Retorna false se não encontrou TracerPid ou se o valor for 0
}

pub fn is_debug( mut env: JNIEnv,_class: JClass,context: JObject,) -> bool {
    let is_debuggable = debug_check(&mut env, &_class, &context);
    let is_debugger_connected = detect_debugger(&mut env, &_class);
    let has_tracer = has_tracer_pid();
    let can_kill = try_kill();
    let any_detected = is_debuggable != 0 || is_debugger_connected != 0 || has_tracer || can_kill;
  
    any_detected
}


/*
fn detect_thread_cpu_time_nanos(env: &mut JNIEnv, _class: &JClass) -> jboolean {
    let debug_class = env.find_class("android/os/Debug").unwrap();

    let start = match env.call_static_method(&debug_class, "threadCpuTimeNanos", "()J", &[]) {
        Ok(result) => result.j().unwrap_or(0),
        Err(_) => return 0 as jboolean,
    };

    for _ in 0..10_000_000 {
        std::hint::spin_loop();
    }

    let stop = match env.call_static_method(&debug_class, "threadCpuTimeNanos", "()J", &[]) {
        Ok(result) => result.j().unwrap_or(0),
        Err(_) => return 0 as jboolean,
    };

    (stop - start >= 10_000_000) as jboolean
}

fn detect_developer_mode(env: &mut JNIEnv, _class: &JClass, context: &JObject) -> jboolean {
    let content_resolver = match env.call_method(context, "getContentResolver", "()Landroid/content/ContentResolver;", &[]) {
        Ok(resolver) => resolver.l().unwrap_or(JObject::null()),
        Err(_) => return 0,
    };

    if content_resolver.is_null() {
        return 0;
    }

    let setting_value = match env.call_static_method(
        "android/provider/Settings$Global",
        "getInt",
        "(Landroid/content/ContentResolver;Ljava/lang/String;I)I",
        &[(&content_resolver).into(), (&env.new_string("development_settings_enabled").unwrap()).into(), 0.into()],
    ) {
        Ok(value) => value.i().unwrap_or(0),
        Err(_) => return 0,
    };

    (setting_value != 0) as jboolean
}

fn detect_adb_status(env: &mut JNIEnv, _class: &JClass, context: &JObject) -> jboolean {
    let content_resolver = match env.call_method(context, "getContentResolver", "()Landroid/content/ContentResolver;", &[]) {
        Ok(resolver) => resolver.l().unwrap_or(JObject::null()),
        Err(_) => return 0,
    };

    if content_resolver.is_null() {
        return 0;
    }

    let adb_enabled = match env.call_static_method(
        "android/provider/Settings$Global",
        "getInt",
        "(Landroid/content/ContentResolver;Ljava/lang/String;I)I",
        &[(&content_resolver).into(), (&env.new_string("adb_enabled").unwrap()).into(), 0.into()],
    ) {
        Ok(value) => value.i().unwrap_or(0),
        Err(_) => return 0,
    };

    (adb_enabled != 0) as jboolean
}*/
