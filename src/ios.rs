use objc::{msg_send, sel, sel_impl};
use objc::runtime::{Class, Object, BOOL, YES};

pub fn is_camera_permission_granted() -> bool {
    let status = Class::get("AVCaptureDevice").map_or(NO, |av_capture_device| {
        let authorization_status: *const Object =
            msg_send![av_capture_device, authorizationStatusForMediaType:"vide"];
        let status: i32 = msg_send![authorization_status, intValue];
        status as BOOL
    });
    status == YES
}