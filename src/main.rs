use std::io::{stdout, Write};

const P: u8 = (255.0 * 0.80) as u8;
const S: u8 = (255.0 * 0.40) as u8;

const O: u8 = 32;

const COLORS: [(u8, u8, u8); 16] = [
    (0, 0, 0),       // Black
    (P, S, S),  // Red
    (S, P, S), // Green
    (S, S, 0),   // Yellow
    (149, 167, 205), // Blue
    (201, 120, 235), // Magenta
    (0, P, P),   // Cyan
    (255, 255, 255), // White
    (139, 134, 128), // Black (Bright) (middle grey)
    (P + O, S + O, S + O), // Red (Bright)
    (189, 218, 189), // Green (Bright)
    (170, 170, 0),   // Yellow (Bright)
    (135, 175, 223), // Blue (Bright)
    (219, 163, 235), // Magenta (Bright)
    (0, 255, 255),   // Cyan (Bright)
    (P, P, P), // White (Bright)
];

fn main() {
    let mut stdout = stdout();

    // Step 1: Print each color with no background
    println!("Foreground colors only:");
    for (index, &(r, g, b)) in COLORS.iter().enumerate() {
        print!("\x1b[38;2;{};{};{}m {:02} \x1b[0m  ", r, g, b, index);
    }
    println!("\n");

    // Step 2: Print each permutation of foreground and background
    println!("Foreground & Background Permutations:");
    for (fg_index, &(fg_r, fg_g, fg_b)) in COLORS.iter().enumerate() {
        for (bg_index, &(bg_r, bg_g, bg_b)) in COLORS.iter().enumerate() {
            print!(
                "\x1b[38;2;{};{};{}m\x1b[48;2;{};{};{}m {:02}/{:02} \x1b[0m ",
                fg_r, fg_g, fg_b, bg_r, bg_g, bg_b, fg_index, bg_index
            );
        }
        println!();
    }
    stdout.flush().unwrap();
}
