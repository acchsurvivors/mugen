use hex;
use jni::objects::{JByteArray, JObject, JObjectArray};
use jni::JNIEnv;
use sha2::{Digest, Sha256};
use std::io;

//pega o hash do certificado
pub fn calculate_hash_from_cert(env: &mut JNIEnv, context: &JObject) -> Result<String, io::Error> {
    let package_manager = env
        .call_method(
            &context,
            "getPackageManager",
            "()Landroid/content/pm/PackageManager;",
            &[],
        )
        .expect("Failed to get PackageManager")
        .l()
        .expect("Invalid PackageManager object");

    let package_name = env
        .call_method(context, "getPackageName", "()Ljava/lang/String;", &[])
        .expect("Failed to get PackageName")
        .l()
        .expect("Invalid PackageName object");

    let package_info = env
        .call_method(
            package_manager,
            "getPackageInfo",
            "(Ljava/lang/String;I)Landroid/content/pm/PackageInfo;",
            &[(&package_name).into(), (0x40 as i32).into()],
        )
        .expect("Failed to get PackageInfo")
        .l()
        .expect("Invalid PackageInfo object");

    let signatures: JObjectArray = env
        .get_field(
            package_info,
            "signatures",
            "[Landroid/content/pm/Signature;",
        )
        .expect("Failed to get signatures field")
        .l()
        .expect("Invalid signatures field")
        .into();

    let first_signature = env
        .get_object_array_element(signatures, 0)
        .expect("Failed to get first signature");

    let signature_bytes: JByteArray = env
        .call_method(first_signature, "toByteArray", "()[B", &[])
        .expect("Failed to get byte array from signature")
        .l()
        .expect("Invalid byte array from signature")
        .into();

    let bytes = env
        .convert_byte_array(signature_bytes)
        .expect("Failed to convert signature bytes");

    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    let result = hasher.finalize();
    let hex_hash = hex::encode(result);

    Ok(hex_hash)
}
