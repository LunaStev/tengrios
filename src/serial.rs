use uart_16550::SerialPort;
use spin::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    SERIAL1.lock().write_fmt(args).expect("Printing to serial failed");
}

/// Prints formatted text to the host system via the serial interface without a newline.
///
/// This macro is useful for debugging in environments without standard output,
/// such as operating system kernels or bare-metal applications. The output
/// is sent through the serial port defined in the `serial` module.
///
/// # Examples
///
/// ```compile_fail
/// use tengrios::serial_print;
///
/// serial_print!("Initializing module...");
/// ```
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*));
    };
}

/// Prints formatted text to the host system via the serial interface, followed by a newline.
///
/// Similar to [`serial_print!`], this macro is designed for low-level environments where
/// standard output is not available. It automatically appends a newline character at the end
/// of the formatted string, making it suitable for line-by-line logging.
///
/// # Examples
///
/// ```compile_fail
/// use tengrios::serial_println;
///
/// serial_println!("Module {} initialized", "VGA");
/// serial_println!();
/// ```
#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}
