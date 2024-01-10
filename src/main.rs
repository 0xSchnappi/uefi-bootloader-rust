#![no_std]  // 不使用标准库
/*
 * start lang项定义了我们程序的入口点。当程序执行时，运行的第一个函数不是主函数，
 * 而是一个入口函数，入口函数的功能是为程序构建运行环境，例如为程序设置堆栈。
 * UEFI规范定义了一个名为`EFI_IMAGE_ENTRY_POINT`的函数，该函数是UEFI固件期望将执行切换到的函数。
 * 对于用Rust编写的UEFI应用程序，链接器会查找名为`efi_main`的函数作为入口点。
 */
#![no_main]
// #![feature(abi_efiapi)]

use core::panic::PanicInfo;

mod uefi;
use uefi::*;
/*
 * UEFI Specification,Version2.9
 * typedef
 * EFI_STATUS
 * (EFIAPI *EFI_IMAGE_ENTRY_POINT) (
 *     IN EFI_HANDLE           ImageHandle,
 *     IN EFI_SYSTEM_TABLE     *SystemTable
 * );
 * 
 */

#[no_mangle] // 告诉编译器不要修改其名称
pub extern "efiapi" fn efi_main(handle: ImageHandle, system_table: *const SystemTable) {
    let string = "hello\n\r";
    for character in string.chars() {
        let mut buffer: [u16; 1] = [0]; // 创建数组元素类型为u16,个数为1个
        let utf16 = character.encode_utf16(&mut buffer);
        unsafe {
            let status =
                ((*(*system_table).output).output_string)((*system_table).output, &utf16[0]);
        }
    }

    let string_arr = ['h' as u16, 'i' as u16, '!' as u16, '\n' as u16, '\0' as u16];

    unsafe {
        let status =
            ((*(*system_table).output).output_string)((*system_table).output, &string_arr[0]);
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // !表示该函数永不返回
    // 目前如果程序出现紧急情况，我们将简单的循环
    loop {}
}
