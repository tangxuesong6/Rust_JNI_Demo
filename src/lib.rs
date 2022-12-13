use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use android_logger_lite as log;
use jni::JNIEnv;
use jni::objects::{JClass, JObject, JString, JValue};
use jni::signature::Primitive::Void;
use jni::signature::ReturnType;
use jni::strings::JNIString;
use jni::sys::{jbyteArray, jobjectArray, jstring};

#[no_mangle]
pub extern "system" fn Java_com_jni_rust_RustNative_getStringFromRust(env: JNIEnv, _: JClass) -> jstring {
    let output = env.new_string("hi bro from rust").unwrap();
    output.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_jni_rust_RustNative_getByteFromString(env: JNIEnv, _: JClass, java_str: JString) -> jbyteArray {
    let input: String = env.get_string(java_str).unwrap().into();
    let input_array = input.as_bytes();
    let output = env.byte_array_from_slice(input_array).unwrap();
    output
}

#[no_mangle]
pub extern "system" fn Java_com_jni_rust_RustNative_callLog(_: JNIEnv, _: JClass) {
    let tag = "rust_tag";
    log::v(tag.to_string(), "hello v".to_string());
    log::d(tag.to_string(), "hello d".to_string());
    log::i(tag.to_string(), "hello i".to_string());
    log::w(tag.to_string(), "hello w".to_string());
    log::e(tag.to_string(), "hello e".to_string());
}

#[no_mangle]
pub extern "system" fn Java_com_jni_rust_RustNative_syncCallback(env: JNIEnv, _: JClass, callback: JObject) {
    let hello = "hello syncCallback";
    let jni_string_hello = JNIString::from(hello);
    let j_string_hello = env.new_string(jni_string_hello).unwrap();
    let j_value_hello = JValue::from(j_string_hello);

    env.call_method(callback, "onStringCallback", "(Ljava/lang/String;)V", &[j_value_hello]).unwrap();
    env.call_method(callback, "onVoidCallback", "()V", &[]).unwrap();
}

#[no_mangle]
pub extern "system" fn Java_com_jni_rust_RustNative_asyncCallback(env: JNIEnv, _: JClass, callback: JObject) {
    let jvm = env.get_java_vm().unwrap();
    let callback = env.new_global_ref(callback).unwrap();
    let (tx, rx) = mpsc::channel();

    let _ = thread::spawn(move || {
        tx.send(()).unwrap();
        let env = jvm.attach_current_thread().unwrap();
        let hello = "hello syncCallback";
        let jni_string_hello = JNIString::from(hello);
        let j_string_hello = env.new_string(jni_string_hello).unwrap();
        let j_value_hello = JValue::from(j_string_hello);

        for _i in 0..6 {
            env.call_method(&callback, "onStringCallback", "(Ljava/lang/String;)V", &[j_value_hello]).unwrap();
            thread::sleep(Duration::from_millis(2000));
        }
    });
    rx.recv().unwrap();
}

#[no_mangle]
pub unsafe extern fn Java_com_jni_rust_RustNative_singleton(env: JNIEnv, _: JClass) {
    let clz = match env.find_class("com/jni/rust/NativeSingleton") {
        Ok(class) => { class }
        Err(_) => {
            panic!("can't find class NativeSingleton");
        }
    };
    let instance_method_id = match env.get_static_method_id(clz, "getInstance", "()Lcom/jni/rust/NativeSingleton;") {
        Ok(instance_method_id) => { instance_method_id }
        Err(_) => {
            panic!("can't find method NativeSingleton.getInstance");
        }
    };
    let instance = match env.call_static_method_unchecked(clz, instance_method_id, ReturnType::Object, &[]) {
        Ok(obj) => { obj }
        Err(_) => {
            panic!("can't call method getInstance");
        }
    };
    let instance_obj = JObject::from(instance.l().unwrap());
    let log_identity_hashcode = match env.get_method_id(clz, "logIdentityHashCode", "()V") {
        Ok(log_identity_hashcode) => { log_identity_hashcode }
        Err(_) => {
            panic!("can't call method logIdentityHashCode");
        }
    };
    env.call_method_unchecked(instance_obj, log_identity_hashcode, ReturnType::Primitive(Void), &[]).unwrap();
}

#[no_mangle]
pub extern "system" fn Java_com_jni_rust_RustNative_getSignatureNormal(env: JNIEnv, _: JClass) -> jstring {
    let activity_thread_clz = env.find_class("android/app/ActivityThread").unwrap();
    let application_value = env.call_static_method(activity_thread_clz, "currentApplication", "()Landroid/app/Application;", &[]).unwrap();
    let application = JObject::try_from(application_value).unwrap();

    //packageName
    let package_name_value = env.call_method(application, "getPackageName", "()Ljava/lang/String;", &[]).unwrap();
    //JValue to JString
    let pkg_name = JString::from(package_name_value.l().unwrap());
    //JString to rust String
    let pkg_name: String = env.get_string(pkg_name).unwrap().into();
    log::d("sign".to_string(), format!("package name = {}", pkg_name));

    //PackageManager.GET_SIGNATURES
    let pm_signatures = JValue::from(64);
    let package_manager = env.call_method(application, "getPackageManager", "()Landroid/content/pm/PackageManager;", &[]).unwrap();
    let package_info = env.call_method(package_manager.l().unwrap(), "getPackageInfo", "(Ljava/lang/String;I)Landroid/content/pm/PackageInfo;", &[package_name_value, pm_signatures]).unwrap();
    let signatures_value = env.get_field(package_info.l().unwrap(), "signatures", "[Landroid/content/pm/Signature;").unwrap();

    //JValue to JObject
    let signature_array_obj = signatures_value.l().unwrap();

    //JObject to jarray
    let signature_array = jobjectArray::from(signature_array_obj.cast());

    let signature_obj = env.get_object_array_element(signature_array, 0).unwrap();
    let sign_value = env.call_method(signature_obj, "toByteArray", "()[B", &[]).unwrap();

    let message_digest_clz = env.find_class("java/security/MessageDigest").unwrap();
    let md5 = env.new_string("md5").unwrap();

    //JString to JValue
    let md5 = JValue::from(md5);
    let message_digest_value = env.call_static_method(message_digest_clz, "getInstance",
                                                      "(Ljava/lang/String;)Ljava/security/MessageDigest;", &[md5]).unwrap();
    let _reset = env.call_method(message_digest_value.l().unwrap(), "reset", "()V", &[]).unwrap();
    let _update = env.call_method(message_digest_value.l().unwrap(), "update", "([B)V", &[sign_value]).unwrap();
    let digest_value = env.call_method(message_digest_value.l().unwrap(), "digest", "()[B", &[]).unwrap();

    let digest_array = jbyteArray::from(digest_value.l().unwrap().cast());

    //jarray to Vec
    let digest_array = env.convert_byte_array(digest_array).unwrap();
    //get hex
    let hex_sign: String = digest_array.iter()
        .map(|b| format!("{:02x}", b).to_string())
        .collect::<Vec<String>>().join("");
    log::d("sign".to_string(), format!("{}", hex_sign));

    let hex_sign = JNIString::from(hex_sign);
    let hex_sign = env.new_string(hex_sign).unwrap();
    hex_sign.into_raw()
}




