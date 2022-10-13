#[cfg(target_os="android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;
    use super::*;
    use self::jni::JNIEnv;
    use self::jni::objects::{JClass, JString};
    use self::jni::sys::{jstring};

    #[no_mangle]
    pub unsafe extern fn Java_com_enel_platform_rustlibrary_rust_Hello_greeting(env: JNIEnv, _: JClass, java_name: JString) -> jstring {

        let name: String = env.get_string(java_name).expect("invalid pattern string").into();
        let mut greeting_string: String = "Hello, ".to_owned();

        greeting_string.push_str(&name);
        greeting_string.push_str(", welcome to Rust!");
        env.new_string(greeting_string).unwrap().into_inner()
    }
}