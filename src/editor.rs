use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::{self, Read};

pub struct Editor {}

impl Editor {
    pub fn default() -> Self {
        Self {}
    }

    pub fn run(&self) {
        enable_raw_mode().unwrap();
        let mut byte_aux: u8;
        let mut byte_char: char;
        for byte in io::stdin().bytes() {
            match byte {
                Ok(b) => byte_aux = b,
                Err(err) => panic!("{err}"),
            }
            byte_char = byte_aux as char;
            if byte_char.is_control() {
                println!("Binary: {0:08b} ASCII: {0:#03} \r", byte_aux);
            } else {
                println!(
                    "Binary: {0:08b} ASCII: {0:#03} Character: {1:#?}\r",
                    byte_aux, byte_char
                );
            }

            if byte_aux.eq(&017) {
                break;
            }
        }
        disable_raw_mode().unwrap();
    }

    pub fn draw_rows(&self) {
        let size: (u16, u16) = match crossterm::terminal::size() {
            Ok(result) => result,
            Err(err) => panic!("{err}"),
        };

        //let colum = size.0;
        let row = size.1;

        for c in 0..row {
            crossterm::cursor::MoveTo(0, c);
            println!("~");
        }
    }
}
