#[cfg(target_os = "android")]
pub mod android;

#[cfg(target_os = "ios")]
pub mod ios;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Permission {
    Camera,
}

pub trait Permissions {
    fn is_granted(&self, permission: Permission) -> bool;
}
