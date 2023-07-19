static VGA_TEXT_BUFFER: isize = 0xB8000;

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
	chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT]
	// [[T; #cols]; #rows]
}

pub struct Writer {
	column_position: usize, // Program iterates through each column
	color_code: ColorCode,
	buffer: &'static mut Buffer // Must exist throughout the program
}

impl Writer {
	fn new_line(&mut self) {} // Requires a mutable borrow to itself

	pub fn write_byte(&mut self, byte: u8) {
		match byte {
			b'\n' => self.new_line(),

			byte => {
				if self.column_position >= BUFFER_WIDTH {
					// Must move to next line when at the border
					self.new_line();
				}

				let row = BUFFER_HEIGHT - 1; // TODO: Why is it deincremented?
				let col = self.column_position;
				
				let color_code = self.color_code; // Copies the color_code
				self.buffer.chars[row][col] = ScreenChar {
					ascii_character: byte,
					color_code,
				};

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
}

// Example case:
pub fn print_ex(string: &str) {
	let mut writer = Writer {
		column_position: 0,
		color_code: ColorCode::new(Color::Blue, Color::Black),
		// A pointer to the memory address of the VGA_BUFFER
		buffer: unsafe {&mut *(VGA_TEXT_BUFFER as *mut Buffer)}

	};

	writer.write_string(string)
}

pub fn print_ex_char(chars: &char) {
	let mut writer = Writer {
		column_position: 0,
		color_code: ColorCode::new(Color::Blue, Color::Black),
		// A pointer to the memory address of the VGA_BUFFER
		buffer: unsafe {&mut *(VGA_TEXT_BUFFER as *mut Buffer)}

	};

	writer.write_byte(*chars as u8); // Since ascii_character is in u8
}