//! 关于UEFI接口的定义，请参考官网文档https://uefi.org/specifications
//! 或着参考UEFI Specification Version 2.9 https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf

// use std::ffi::c_void;
// pub type ImageHandle = *const c_void;
// // or
pub type ImageHandle = u64;
//whichever is more pleasing to the eyes
pub type Status = usize;

#[repr(C)]
pub struct SystemTable {
    header: [u8; 24],
    firmware_bendor: u64,
    firmware_revision: u32,
    input_handle: ImageHandle,
    input: u64,
    output_handle: ImageHandle,
    pub output: *const TextOutputProtocol,
    error_handle: ImageHandle,
    error: u64,
    runtime: u64,
    boot: u64,
    no_of_entries: usize,
    config_table: u64,
}

#[repr(C)]
pub struct TextOutputProtocol {
    reset: u64,
    pub output_string: OutputString,
    test_output: u64,
    query_mode: u64,
    set_mode: u64,
    set_attribute: u64,
    pub clear_screen: u64,
    set_cursor_position: u64,
    enable_cursor: u64,
    mode: u64,
}

pub type OutputString =
    extern "efiapi" fn(output_protocol: *const TextOutputProtocol, string: *const u16) -> Status;


