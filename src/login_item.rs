use core_foundation_sys::base::Boolean;
use core_foundation_sys::string::CFStringRef;

extern "C" {
    pub fn SMLoginItemSetEnabled(identifier: CFStringRef, enabled: Boolean) -> Boolean;
}
