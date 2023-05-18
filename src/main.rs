use colored::*;

const ASCII_COUNT: usize = 127;
const CHARS: [&str; ASCII_COUNT] = ["NUL", "SOH", "STX", "ETX", "EOT", "ENQ", "ACK", "BEL", "BS", "HT", "LF", "VT", "FF", "CR", "SO", "SI", "DLE", "DC1", "DC2", "DC3", "DC4", "NAK", "SYN", "ETB", "CAN", "EM", "SUB", "ESC", "FS", "GS", "RS", "US", "!", "\"", "#", "$", "%", "&", "'", "(", ")", "*", "+", ",", "-", ".", "/", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", ":", ";", "<", "=", ">", "?", "@", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "[", "\\", "]", "^", "_", "`", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "{", "|", "}", "~", "DEL"];

const LINES: usize = 16;
const COLUMNS: usize = (ASCII_COUNT / LINES) + if ASCII_COUNT % LINES > 0 { 1 } else { 0 };

fn main() {
    println!();

    for current_line in 0..LINES {
        for current_column in 0..COLUMNS {
            let index = current_line + current_column * LINES;
            if index < ASCII_COUNT {
                let ascii_to_print = format!("\t{} {}", index.to_string().green(), CHARS[index]);
                print!("{}", ascii_to_print);
            }
        }
        println!();
    }
    println!();
}