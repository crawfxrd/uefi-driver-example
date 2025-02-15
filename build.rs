// SPDX-License-Identifier: BSD-2-Clause-Patent
// SPDX-FileCopyrightText: 2025 System76, Inc.

use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();
    if target.ends_with("-unknown-uefi") {
        println!("cargo::rustc-link-arg=/subsystem:efi_boot_service_driver");
    }
}
