use std::io::{stdout, Write};

const P: u8 = (255.0 * 0.80) as u8;
const S: u8 = (255.0 * 0.40) as u8;

const O: u8 = 35;

const COLOR_NAMES: [&str; 16] = [
    "Black", "Red", "Green", "Yellow", "Blue", "Magenta", "Cyan", "White",
    "Bright Black", "Bright Red", "Bright Green", "Bright Yellow",
    "Bright Blue", "Bright Magenta", "Bright Cyan", "Bright White"
];

// Kleene Purple "#211930"
const COLORS: [(u8, u8, u8); 16] = [
    (0, 0, 0), // Black
    (P, S, S), // Red
    (S, P, S), // Green
    (P, P, 0), // Yellow
    (S, S + O, P), // Blue
    (P, S, P), // Magenta
    (0, P, P), // Cyan
    (139 + O, 134 + O, 128 + O), // White
    (139, 134, 128), // Black (Bright) (middle grey)
    (P + O, S + O, S + O), // Red (Bright)
    (S + O, P + O, S + O), // Green (Bright)
    (P + O, P + O, 0),   // Yellow (Bright)
    (S + O, S + 2 * O, P + O), // Blue (Bright)
    (P + O, S + O, P + O), // Magenta (Bright)
    (0, P + O, P + O),   // Cyan (Bright)
    // `Black` is true black and `White (Bright)` is true white, that seems to be correct, e.g.,
    // `tig` uses `White` for unselected and `White (Bright)` for selected. Without true white as
    // `White (Bright)` this means the selected commit is hard to read in `tig`.
    (255, 255, 255), // White (Bright)
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

    // Step 3: Print the hex values at the end
    println!("\nHex Values:");
    for (index, &(r, g, b)) in COLORS.iter().enumerate() {
        let hex = format!("#{:02X}{:02X}{:02X}", r, g, b);
        println!("{} : {}", hex, COLOR_NAMES[index]);
    }
}
