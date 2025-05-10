#![no_main]
#![no_std]

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use cortex_m_rt::entry;
use microbit::hal::Delay;

// Randomness source: https://blog.orhun.dev/zero-deps-random-in-rust/#:~:text=Using%20rand,is%20using%20the%20rand%20crate.&text=Generates%20random%20numbers%20with%20help,thread%20has%20an%20initialized%20generator.
use nanorand::wyrand::WyRand; // Source: ChatGPT
use nanorand::Rng;

use embedded_hal::digital::InputPin;
#[rustfmt::skip]
use microbit::{
    board::Board,
    display::blocking::Display,
};

#[entry]
// Source from the template: (template mb2): https://github.com/pdx-cs-rust-embedded/mb2-template
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut display = Display::new(board.display_pins);
    let mut delay = Delay::new(board.SYST);

    let mut buttons = board.buttons; // Assign the board's buttons to var buttons.

    // Create a random value with a seed:
    let mut rng = WyRand::new_seed(35435);

    // 1) Initialize the board with all zeros:
    let mut fb: [[u8; 5]; 5] = [[0; 5]; 5];

    loop {
        // When the A button is pressed, randomize the board:
        if buttons.button_a.is_low().unwrap() {
            rprintln!("Button A pressed! Board Randomized.");
            // When A is pushed, generate a new random board each frame.
            for y in 0..5 {
                for x in 0..5 {
                    fb[y][x] = rng.generate_range(0..=1);
                }
            }
        }
        
        // Convert fb (u8s) to bools for the display:
        let mut leds = [[false; 5]; 5];
        for row in 0..5 {
            for col in 0..5 {
                leds[row][col] = fb[row][col] == 1;
            }
        }

        // Show the frame for 100 ms:
        let mut leds_u8 = [[0u8; 5]; 5];
        for row in 0..5 {
            for col in 0..5 {
                leds_u8[row][col] = if leds[row][col] { 255 } else { 0 };
            }
        }

        // Run the game at 10 frames per second (update once per 100ms):
        // display.show(&mut delay, image_data, duration_ms);
        display.show(
            &mut delay, /* image_data: */ leds_u8, /* ms: */ 100,
        ); // Source: https://docs.rs/microbit-v2/latest/microbit/display/blocking/index.html
    }
}
