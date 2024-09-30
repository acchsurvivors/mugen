use jni::objects::JString;
use jni::JNIEnv;
use std::process::Command;

//verifica se o app está congelado por comandos
pub fn is_frozen(env: &mut JNIEnv) -> bool {
    let context = env
        .call_static_method(
            "android/app/ActivityThread",
            "currentApplication",
            "()Landroid/app/Application;",
            &[],
        )
        .expect("Failed to get application context")
        .l()
        .expect("Invalid context object");

    let application_info = env
        .call_method(
            context,
            "getApplicationInfo",
            "()Landroid/content/pm/ApplicationInfo;",
            &[],
        )
        .expect("Failed to get application info")
        .l()
        .expect("Invalid application info object");

    let package_name_obj = env
        .get_field(application_info, "packageName", "Ljava/lang/String;")
        .expect("Failed to get packageName field")
        .l()
        .expect("Invalid packageName object");

    let package_name: String = env
        .get_string(&JString::from(package_name_obj))
        .expect("Failed to convert packageName to string")
        .into();

    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("dumpsys package {}", package_name))
        .output()
        .expect("Failed to execute command");

    let result = String::from_utf8_lossy(&output.stdout);
    result.contains("enabled=false")
}
