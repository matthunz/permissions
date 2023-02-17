use crate::{Permission, Permissions};
use objc::runtime::{Class, Object, BOOL, YES};
use objc::{msg_send, sel, sel_impl};

pub struct IosPermissions {}

impl Permissions for IosPermissions {
    fn is_granted(&self, permission: Permission) -> bool {
        match permission {
            Permission::Camera => {
                let status = Class::get("AVCaptureDevice").map_or(NO, |av_capture_device| {
                    let authorization_status: *const Object =
                        msg_send![av_capture_device, authorizationStatusForMediaType:"vide"];
                    let status: i32 = msg_send![authorization_status, intValue];
                    status as BOOL
                });
                status == YES
            }
        }
    }
}
