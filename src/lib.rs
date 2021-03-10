#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::os::raw::c_uint;

// NOTE: Copied from <evdi_lib.c>, MUST be kept in sync.

/// A kernel module is compatible if its major version is equal to
/// [EVDI_MODULE_COMPATIBILITY_VERSION_MAJOR] and its minor version is >= to
/// [EVDI_MODULE_COMPATIBILITY_VERSION_MINOR]
pub const EVDI_MODULE_COMPATIBILITY_VERSION_MAJOR: u32 = 1;
/// A kernel module is compatible if its major version is equal to
/// [EVDI_MODULE_COMPATIBILITY_VERSION_MAJOR] and its minor version is >= to
/// [EVDI_MODULE_COMPATIBILITY_VERSION_MINOR]
pub const EVDI_MODULE_COMPATIBILITY_VERSION_MINOR: u32 = 9;

pub const EVDI_STATUS_AVAILABLE: c_uint = evdi_device_status_AVAILABLE;
pub const EVDI_STATUS_UNRECOGNIZED: c_uint = evdi_device_status_UNRECOGNIZED;
pub const EVDI_STATUS_NOT_PRESENT: c_uint = evdi_device_status_NOT_PRESENT;

include!("./bindings.rs");

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn evdi_check_device_for_not_present() {
        let status = unsafe {
            evdi_check_device(42)
        };
        assert_eq!(status, EVDI_STATUS_NOT_PRESENT)
    }

    #[test]
    fn is_correct_version() {
        let mut version = evdi_lib_version {
            version_major: -1,
            version_minor: -1,
            version_patchlevel: -1
        };

        unsafe {
            evdi_get_lib_version(&mut version)
        }

        assert_eq!(version.version_major, 1);
        assert_eq!(version.version_minor, 9);
        assert_eq!(version.version_patchlevel, 1);
    }
}
