use jni::sys::jboolean;
use jni::JNIEnv;
mod debug;
mod emulator;
mod general;
mod hook;
mod network;
mod repacking;
mod root;
mod utilit;
mod ml;
mod ml_output;
use hex;
use hex::{decode, encode};
use jni::objects::{JClass, JObject, JString};

#[no_mangle]
pub extern "C" fn Java_com_example_mugen_Mugen_00024Mugen_detect_1root(
    _: JClass,
) -> jboolean {   
    root::is_root() as jboolean
}

#[no_mangle]
pub extern "C" fn Java_com_example_mugen_Mugen_00024Mugen_isEmulator(
    _: JClass,
) -> jboolean {
    emulator::is_emulator() as jboolean
}

#[no_mangle]
pub extern "C" fn Java_com_example_mugen_Mugen_00024Mugen_performDebugChecks(
    env: JNIEnv,
    _class: JClass,
    context: JObject,
) -> jboolean {
   debug::is_debug(env,_class,context,) as jboolean
}

//detecção de anti repack
#[no_mangle]
pub extern "C" fn Java_com_example_mugen_Mugen_00024Mugen_Checks_1repack(
    mut env: JNIEnv,
    _class: JClass,
    context: JObject,
    encrypted_hash: JString,
) -> jboolean {
    // Convert Java String to Rust String
    let encrypted_hash: String = match env.get_string(&encrypted_hash) {
        Ok(hash) => hash.into(),
        Err(_) => return jni::sys::JNI_TRUE,
    };

    // Decode encrypted hash from hex to bytes
    let encrypted_hash_bytes = match decode(&encrypted_hash) {
        Ok(bytes) => bytes,
        Err(_) => return jni::sys::JNI_TRUE,
    };

    // Decrypt the hash
    let decrypted_hash_bytes = utilit::decrypt(&encrypted_hash_bytes);

    // Calculate the SHA-256 hash of the current APK using the provided context
    let current_hash = match repacking::calculate_hash_from_cert(&mut env, &context) {
        Ok(hash) => hash,
        Err(_) => return jni::sys::JNI_TRUE,
    };

    // Decode the current hash from hex to bytes
    let current_hash_bytes = match decode(&current_hash) {
        Ok(bytes) => bytes,
        Err(_) => return jni::sys::JNI_TRUE,
    };

    // Encode the decrypted hash bytes and the current hash bytes to hex
    let decrypted_hex = encode(&decrypted_hash_bytes);
    let current_hex = encode(&current_hash_bytes);

    if decrypted_hex == current_hex {
        jni::sys::JNI_FALSE
    } else {
        jni::sys::JNI_TRUE
    }
}

#[no_mangle]
pub extern "C" fn Java_com_example_mugen_Mugen_00024Mugen_checkSecurityIssues(
    mut env: JNIEnv,
) -> jboolean {
    let is_burp_detected = network::burpcheck();
    let is_frida_port = hook::check_frida_port();
    let is_proxy_detected = network::detect_proxy();
    let is_frida_process = hook::check_frida_process();
    let is_suspicious_files = hook::check_suspicious_files();
    let is_redirects = network::check_redirects();
    let is_frozen_detected = general::is_frozen(&mut env);
    //let is_frida_on_memory = hook::check_for_frida();
   // let is_frida_termination = hook::check_for_frida_termination();

    let any_detected = is_burp_detected
        || is_frida_port
        || is_proxy_detected
        || is_frida_process
        || is_suspicious_files
        || is_redirects
        || is_frozen_detected;
       // || is_frida_on_memory;
       // || is_frida_termination;

    any_detected as jboolean
}

#[no_mangle]
pub extern "C" fn Java_com_example_mugen_Mugen_00024Mugen_decript(
    mut env: JNIEnv,
    encrypted_sslhash: JString,
    real_hash: JString
) -> jboolean {
    // Convert JString to Rust String
    let encrypted_sslhash: String = env.get_string(&encrypted_sslhash)
        .expect("Couldn't get Java string!")
        .into();

    let real_hash: String = env.get_string(&real_hash)
        .expect("Couldn't get Java string!")
        .into();

    // Decode encrypted hash from hex to bytes
    let encrypted_sslhash_bytes = hex::decode(&encrypted_sslhash)
        .expect("Failed to decode hex string");

    // Decrypt the hash
    let decrypted_hash_bytes = utilit::decrypt(&encrypted_sslhash_bytes);

    // Encode the decrypted hash bytes to hex
    let decrypted_hex = hex::encode(&decrypted_hash_bytes);

    // Compare the decrypted hash with the real hash
    if decrypted_hex == real_hash {
        jni::sys::JNI_TRUE  
    } else {
        jni::sys::JNI_FALSE 
    }
}


#[no_mangle]
pub extern "system" fn Java_com_example_mugen_Mugen_00024Mugen_predict() -> jboolean {
    //pega as propriedades 
    let propriets = ml::get_android_properties();
    //carrego os dados  
    let (features, labels) = ml::carregar_dados();
    //treino o modelo
    let (modelo, _) = ml::treinar_modelo(features,labels);
    //previsão
    let resultado = ml::prever_ambiente(propriets, &modelo);

    if resultado {
        jni::sys::JNI_TRUE
    }else{
        jni::sys::JNI_FALSE
    }

}