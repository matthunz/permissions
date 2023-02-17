use std::ptr;

use android_activity::AndroidApp;
use jni::{
    objects::{JClass, JObject, JValue},
    sys::{jboolean, jobject, jstring},
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

pub fn request(android_app: &AndroidApp, permission: &str) {
    let vm = unsafe { JavaVM::from_raw(android_app.vm_as_ptr() as _) }.unwrap();
    let mut env = vm.attach_current_thread().unwrap();

    let j_permission = env.new_string(permission).unwrap();

    let permissions: Vec<jstring> = vec![j_permission.into_raw()];
    let permissions_array = env
        .new_object_array(
            permissions.len() as i32,
            "java/lang/String",
            JObject::null(),
        )
        .unwrap();
    for (i, permission) in permissions.into_iter().enumerate() {
        env.set_object_array_element(&permissions_array, i as i32, unsafe {
            JObject::from_raw(permission)
        })
        .unwrap();
    }

    let activity = unsafe { JObject::from_raw(android_app.activity_as_ptr() as jobject) };
    let res = env
        .call_method(
            activity,
            "requestPermissions",
            "([Ljava/lang/String;I)V",
            &[JValue::Object(&permissions_array), JValue::Int(3)],
        )
        .unwrap();
}
