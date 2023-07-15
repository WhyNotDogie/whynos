use std::{env, fs};

fn main() {
    let current_exe = env::current_exe().unwrap();
    let uefi_target = current_exe.with_file_name("uefi.img");
    let bios_target = current_exe.with_file_name("bios.img");

    fs::copy(option_env!("UEFI_IMAGE").unwrap(), &uefi_target).unwrap();
    fs::copy(option_env!("BIOS_IMAGE").unwrap(), &bios_target).unwrap();

    println!("UEFI disk image at {}", uefi_target.display());
    println!("BIOS disk image at {}", bios_target.display());
}