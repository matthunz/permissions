use android_activity::AndroidApp;

#[no_mangle]
fn android_main(android_app: AndroidApp) {
    let permission = "android.permission.CAMERA";
    if !permissions::android::is_granted(permission) {
        permissions::android::request(&android_app, permission);
    }

    loop {}
}
