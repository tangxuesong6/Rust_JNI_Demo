mod android_log_lite;
use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::{jbyteArray, jstring};
use crate::android_log_lite as log;

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


