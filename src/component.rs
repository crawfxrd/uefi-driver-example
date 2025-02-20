// SPDX-License-Identifier: BSD-2-Clause-Patent
// SPDX-FileCopyrightText: 2025 System76, Inc.

// References:
// - UEFI 2.11: 11.5 EFI Component Name Protocol
// - UEFI 2.11: Appendix M Formats - Language Codes and Language Code Arrays

use uefi::cstr16;
use uefi::CStr16;
use uefi_raw::protocol::driver::ComponentName2Protocol;
use uefi_raw::Handle;
use uefi_raw::Status;

const DRIVER_NAME: &CStr16 = cstr16!("Example Driver");

// EFI_COMPONENT_NAME2_GET_DRIVER_NAME
unsafe extern "efiapi" fn get_driver_name(
    _this: *const ComponentName2Protocol,
    language: *const u8,
    driver_name: *mut *const u16,
) -> Status {
    if language.is_null() || driver_name.is_null() {
        return Status::INVALID_PARAMETER;
    }

    // TODO: Implement lookup to handle multiple languages.
    unsafe { *driver_name = DRIVER_NAME.to_u16_slice_with_nul().as_ptr() };
    Status::SUCCESS
}

// EFI_COMPONENT_NAME2_GET_CONTROLLER_NAME
unsafe extern "efiapi" fn get_controller_name(
    _this: *const ComponentName2Protocol,
    controller_handle: Handle,
    _child_handle: Handle,
    language: *const u8,
    controller_name: *mut *const u16,
) -> Status {
    if controller_handle.is_null() || language.is_null() || controller_name.is_null() {
        return Status::INVALID_PARAMETER;
    }

    Status::UNSUPPORTED
}

// EFI_COMPONENT_NAME2_PROTOCOL
// NOTE: This is a static mutable to avoid the following error:
//     `*const u8` cannot be shared between threads safely
pub static mut COMPONENT_NAME: ComponentName2Protocol = ComponentName2Protocol {
    get_driver_name,
    get_controller_name,
    supported_languages: c"en".as_ptr() as *const u8,
};
