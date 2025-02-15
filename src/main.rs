// SPDX-License-Identifier: BSD-2-Clause-Patent
// SPDX-FileCopyrightText: 2025 System76, Inc.

// UEFI driver example

#![no_main]
#![no_std]

mod binding;
mod component;

use crate::binding::DRIVER_BINDING;
use crate::component::COMPONENT_NAME;
use uefi::prelude::*;
use uefi::proto::loaded_image::LoadedImage;
use uefi::{guid, Guid};
use uefi_raw::protocol::driver::ComponentName2Protocol;
use uefi_raw::protocol::driver::DriverBindingProtocol;

// XXX: Get from INF? Generate INF using this?
pub const DRIVER_GUID: Guid = guid!("e85d2ad7-a062-4fbd-b704-d12e9b0595eb");

// EFI_IMAGE_UNLOAD
// References:
// - UEFI 2.11: 7.3.13 EFI_BOOT_SERVICES.DisconnectController()
extern "efiapi" fn unload(image: Handle) -> Status {
    let hbuffer = boot::locate_handle_buffer(boot::SearchType::ByProtocol(&DRIVER_GUID));
    if let Ok(handles) = hbuffer {
        for handle in handles.iter() {
            let ret = boot::disconnect_controller(*handle, Some(image), None);
            if ret.is_err() {
                log::warn!("failed to disconnect controller: {}", ret.status());
            }
        }
    }

    unsafe {
        boot::uninstall_protocol_interface(
            image,
            &ComponentName2Protocol::GUID,
            core::ptr::addr_of!(COMPONENT_NAME).cast(),
        )
        .unwrap();

        boot::uninstall_protocol_interface(
            image,
            &DriverBindingProtocol::GUID,
            core::ptr::addr_of!(DRIVER_BINDING).cast(),
        )
        .status()
    }
}

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();
    let image = boot::image_handle();

    unsafe {
        DRIVER_BINDING.image_handle = image.as_ptr();
        DRIVER_BINDING.driver_binding_handle = image.as_ptr();
    }

    unsafe {
        boot::install_protocol_interface(
            Some(image),
            &DriverBindingProtocol::GUID,
            core::ptr::addr_of!(DRIVER_BINDING).cast(),
        )
        .unwrap();

        boot::install_protocol_interface(
            Some(image),
            &ComponentName2Protocol::GUID,
            core::ptr::addr_of!(COMPONENT_NAME).cast(),
        )
        .unwrap();
    }

    unsafe {
        let mut loaded_image = boot::open_protocol::<LoadedImage>(
            boot::OpenProtocolParams {
                handle: image,
                agent: image,
                controller: None,
            },
            boot::OpenProtocolAttributes::GetProtocol,
        )
        .unwrap();

        loaded_image.set_unload(unload);
    }

    Status::SUCCESS
}
