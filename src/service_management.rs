use core_foundation_sys::base::Boolean;
use core_foundation_sys::error::CFErrorRef;
use core_foundation_sys::string::CFStringRef;
use security_framework_sys::authorization::AuthorizationRef;

pub const kSMRightBlessPrivilegedHelper: &[u8; 40] = b"com.apple.ServiceManagement.blesshelper\0";
pub const kSMRightModifySystemDaemons: &[u8; 43] = b"com.apple.ServiceManagement.daemons.modify\0";

extern "C" {
    pub static kSMDomainSystemLaunchd: CFStringRef;
    pub static kSMDomainUserLaunchd: CFStringRef;
}

extern "C" {
    pub fn SMJobBless(
        domain: CFStringRef,
        executableLabel: CFStringRef,
        auth: AuthorizationRef,
        error: *mut CFErrorRef,
    ) -> Boolean;
}
