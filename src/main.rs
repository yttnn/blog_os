#![no_std]
#![no_main] // エントリポイントを無効化

mod vga_buffer;

use core::panic::PanicInfo;


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	println!("{}", _info);
	loop {}
}

#[no_mangle] // 名前修飾しないように指定, entry function
pub extern "C" fn _start() -> ! {

	//println!("Hello Wörld{}", "!");
	println!("Hello World{}", "!");

	loop {}
}