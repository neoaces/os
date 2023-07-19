use volatile::Volatile;
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
static VGA_TEXT_BUFFER: isize = 0xB8000;
lazy_static! {
	pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer { 
		// Ref needed for lazy_static
		column_position: 0,
		color_code: ColorCode::new(Color::Yellow, Color::Black),
		buffer: unsafe {&mut *(VGA_TEXT_BUFFER as *mut Buffer)}
	});
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)] // Only requires 4 
pub enum Color {
	// See https://en.wikipedia.org/wiki/VGA_text_mode#:~:text=%5Bedit%5D-,Text%20buffer,-%5Bedit%5D
	Black = 0,
	Blue = 1,
	Green = 2,
	Cyan = 3,
	Red = 4,
	Magenta = 5,
	Brown = 6,
	LightGray = 7,
	DarkGray = 8,
	LightBlue = 9,
	LightGreen = 10,
	LightCyan = 11,
	LightRed = 12,
	LightMagenta = 13,
	Yellow = 14,
	White = 15
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8); 
// Will cause ColorCode to be a newtype of u8
// Data layout is identical to u8

impl ColorCode {
	fn new(foreground: Color, background: Color) -> ColorCode {
		ColorCode((background as u8) << 4 | (foreground as u8))
	}
}

// Full representation of the color portion of one write to the VGA buffer
// 1 buffer = 2 bytes of u8 (Color | byte of ASCII character)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
	ascii_character: u8,
	color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
	// Creates an array of ScreenChars with size BUFFER_WIDTH x BUFFER_HEIGHT
	// Represents the entire screen
	chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT]
	// [[T; #cols]; #rows]
}

pub struct Writer {
	column_position: usize, // Program iterates through each column
	color_code: ColorCode,
	buffer: &'static mut Buffer // Must exist throughout the program
}

impl Writer {
	pub fn write_byte(&mut self, byte: u8) {
		match byte {
			b'\n' => self.new_line(),

			byte => {
				if self.column_position >= BUFFER_WIDTH {
					// Must move to next line when at the border
					self.new_line();
				}
				// Prints from the top left, thus to print at the bottom, needs to decrement 1 using top of row to print char under
				let row = BUFFER_HEIGHT - 1; 
				let col = self.column_position;
				
				let color_code = self.color_code; // Copies the color_code
				self.buffer.chars[row][col].write(ScreenChar {
					ascii_character: byte,
					color_code,
				});

				self.column_position += 1; // Increments the column forward
			}
		}
	}

	pub fn write_string(&mut self, s: &str) {
		for byte in s.bytes() { // Returns an iterator for each byte
			match byte {
				// Printable range is from 0x20 to 0x7e
				0x20..=0x7e | b'\n' => self.write_byte(byte), // space to tilde (~)
				
				_ => self.write_byte(0xfe) // BLACK SQUARE: invalid code
			}
		}
	}

	fn new_line(&mut self) {
		for row in 1..BUFFER_HEIGHT {
			for col in 0..BUFFER_WIDTH {
				let character = self.buffer.chars[row][col].read(); // Looks at the char at nth position
				self.buffer.chars[row-1][col].write(character)
			}
		}

		self.clear_row(BUFFER_HEIGHT - 1);
		self.column_position = 0;
	}

	fn clear_row(&mut self, row: usize) {
		let blank = ScreenChar {
			ascii_character: b' ', // Replaces the row with spaces
			color_code: self.color_code,
		};

		for col in 0..BUFFER_WIDTH {
			self.buffer.chars[row][col].write(blank);
		}
	}
}

impl fmt::Write for Writer { // Allows printing integers and floats
	// Implements the Write trait
	fn write_str(&mut self, s: &str) -> fmt::Result {
		self.write_string(s);
		Ok(())
	}
}

// Definition of println! in std library
// #[macro_export]
// macro_rules! println {
//     () => (print!("\n"));
//     ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
// }
// - Note how it calls the print! macro, and print! calls a function called _print().

#[macro_export]
macro_rules! print {
	($($arg:tt)*) => {
		$crate::vga_buffer::_print(format_args!($($arg)*));
	};
}

#[macro_export]
macro_rules! println {
	() => ($crate::print!("\n"));
	($($arg:tt)*) => {
		$crate::print!("{}\n", format_args!($($arg)*));
	};
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
	use core::fmt::Write;
	WRITER.lock().write_fmt(args).unwrap();
}
// Macros for printing to screen

// Example case: NOT REQUIRED AFTER WRITER STATIC
// pub fn print_ex(string: &str) {
// 	use core::fmt::Write;
// 	let mut writer = Writer {
// 		column_position: 0,
// 		color_code: ColorCode::new(Color::LightRed, Color::Black),
// 		// A pointer to the memory address of the VGA_BUFFER
// 		buffer: unsafe {&mut *(VGA_TEXT_BUFFER as *mut Buffer)}

// 	};

// 	writeln!(writer, "{}", string).unwrap();
// }

// pub fn print_ex_char(chars: &char) {
// 	let mut writer = Writer {
// 		column_position: 0,
// 		color_code: ColorCode::new(Color::Blue, Color::Black),
// 		// A pointer to the memory address of the VGA_BUFFER
// 		buffer: unsafe {&mut *(VGA_TEXT_BUFFER as *mut Buffer)}

// 	};

// 	writer.write_byte(*chars as u8); // Since ascii_character is in u8
// }