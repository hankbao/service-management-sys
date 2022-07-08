#![allow(bad_style)]

#[cfg_attr(
    target_os = "macos",
    link(name = "ServiceManagement", kind = "framework")
)]
extern "C" {}

pub mod errors;
pub mod login_item;
pub mod service_management;
