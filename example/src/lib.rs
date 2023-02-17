use android_activity::AndroidApp;

#[no_mangle]
fn android_main(_android_app: AndroidApp) {
    let permission = "android.permission.CAMERA";
    permissions::is_granted(permission);

    loop {}
}
