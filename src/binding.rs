// SPDX-License-Identifier: BSD-2-Clause-Patent
// SPDX-FileCopyrightText: 2025 System76, Inc.

// References:
// - UEFI 2.11: 11.1 EFI Driver Binding Protocol

use uefi_raw::protocol::device_path::DevicePathProtocol;
use uefi_raw::protocol::driver::DriverBindingProtocol;
use uefi_raw::Handle;
use uefi_raw::Status;

// EFI_DRIVER_BINDING_PROTOCOL_SUPPORTED
unsafe extern "efiapi" fn supported(
    _this: *const DriverBindingProtocol,
    _controller: Handle,
    _remaining: *const DevicePathProtocol,
) -> Status {
    Status::UNSUPPORTED
}

// EFI_DRIVER_BINDING_PROTOCOL_START
unsafe extern "efiapi" fn start(
    _this: *const DriverBindingProtocol,
    _controller: Handle,
    _remaining: *const DevicePathProtocol,
) -> Status {
    Status::SUCCESS
}

// EFI_DRIVER_BINDING_PROTOCOL_STOP
unsafe extern "efiapi" fn stop(
    _this: *const DriverBindingProtocol,
    _controller: Handle,
    _children_number: usize,
    _children: *const Handle,
) -> Status {
    Status::SUCCESS
}

// EFI_DRIVER_BINDING_PROTOCOL
// NOTE: This is a static mutable because `image_handle` and
// `driver_binding_handle` are set when the image is loaded.
pub static mut DRIVER_BINDING: DriverBindingProtocol = DriverBindingProtocol {
    supported,
    start,
    stop,
    version: 1,
    image_handle: core::ptr::null_mut(),
    driver_binding_handle: core::ptr::null_mut(),
};
