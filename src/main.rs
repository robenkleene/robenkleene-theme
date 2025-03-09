use std::io::{stdout, Write};

const COLORS: [(u8, u8, u8); 16] = [
    (0, 0, 0),       // Black
    (128, 0, 0),     // Red
    (0, 128, 0),     // Green
    (128, 128, 0),   // Yellow
    (0, 0, 128),     // Blue
    (128, 0, 128),   // Magenta
    (0, 128, 128),   // Cyan
    (192, 192, 192), // Light Gray
    (128, 128, 128), // Dark Gray
    (255, 0, 0),     // Bright Red
    (0, 255, 0),     // Bright Green
    (255, 255, 0),   // Bright Yellow
    (0, 0, 255),     // Bright Blue
    (255, 0, 255),   // Bright Magenta
    (0, 255, 255),   // Bright Cyan
    (255, 255, 255), // White
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
