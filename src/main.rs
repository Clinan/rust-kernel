#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let message = "Hello, world!";
    let mut uart = Uart::new();
    uart.write_str(message);
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

struct Uart;

impl Uart {
    fn new() -> Uart {
        Uart {}
    }

    fn write_str(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
    }

    fn write_byte(&mut self, byte: u8) {
        // write byte to UART
    }
}

