use android_logger_lite as log;
use jni::JNIEnv;
use jni::objects::{JClass, JObject, JString, JValue};
use jni::strings::JNIString;
use jni::sys::{jbyteArray, jstring};

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



