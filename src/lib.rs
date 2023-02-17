use jni::{
    objects::{JClass, JObject, JValue},
    sys::jboolean,
    JNIEnv, JavaVM,
};

pub fn is_granted(permission: &str) -> bool {
    let cx = ndk_context::android_context();
    let vm = unsafe { JavaVM::from_raw(cx.vm().cast()) }.unwrap();
    let mut env = vm.attach_current_thread().unwrap();

    let j_permission = env.new_string(permission).unwrap();
    let permission_granted = env
        .call_method(
            &unsafe { JObject::from_raw(cx.context() as jni::sys::jobject) },
            "checkSelfPermission",
            "(Ljava/lang/String;)I",
            &[JValue::try_from(&j_permission).unwrap()],
        )
        .unwrap();

    permission_granted.i().unwrap() != -1
}
